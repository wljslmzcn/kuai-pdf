use lopdf::{Document, Object, Dictionary};
use std::path::Path;
use std::collections::HashSet;
use crate::pdf_utils::PdfUtils;

pub struct PageOps;

impl PageOps {
    /// 递归复制对象及其所有引用的对象，返回新文档中的对象ID
    fn copy_object_recursive(
        doc: &Document,
        new_doc: &mut Document,
        obj_id: lopdf::ObjectId,
        copied: &mut HashSet<lopdf::ObjectId>,
        id_map: &mut std::collections::HashMap<lopdf::ObjectId, lopdf::ObjectId>,
    ) -> Option<lopdf::ObjectId> {
        // 如果已经复制过，返回映射后的ID
        if let Some(&new_id) = id_map.get(&obj_id) {
            return Some(new_id);
        }
        if copied.contains(&obj_id) {
            return id_map.get(&obj_id).copied();
        }
        copied.insert(obj_id);

        let obj = match doc.get_object(obj_id) {
            Ok(o) => o.clone(),
            Err(_) => return None,
        };

        // 先递归处理子对象
        let processed_obj = match obj {
            Object::Dictionary(mut dict) => {
                let new_dict = Self::process_dict_refs(doc, new_doc, &mut dict, copied, id_map);
                Object::Dictionary(new_dict)
            }
            Object::Array(arr) => {
                let new_arr = Self::process_array_refs(doc, new_doc, &arr, copied, id_map);
                Object::Array(new_arr)
            }
            Object::Stream(mut stream) => {
                let new_dict = Self::process_dict_refs(doc, new_doc, &mut stream.dict, copied, id_map);
                stream.dict = new_dict;
                Object::Stream(stream)
            }
            other => other,
        };

        let new_id = new_doc.add_object(processed_obj);
        id_map.insert(obj_id, new_id);
        Some(new_id)
    }

    /// 处理字典中的引用
    fn process_dict_refs(
        doc: &Document,
        new_doc: &mut Document,
        dict: &mut Dictionary,
        copied: &mut HashSet<lopdf::ObjectId>,
        id_map: &mut std::collections::HashMap<lopdf::ObjectId, lopdf::ObjectId>,
    ) -> Dictionary {
        let mut new_dict = Dictionary::new();
        for (key, val) in dict.iter() {
            let new_val = Self::process_object_refs(doc, new_doc, val, copied, id_map);
            let _ = new_dict.set(key.clone(), new_val);
        }
        new_dict
    }

    /// 处理数组中的引用
    fn process_array_refs(
        doc: &Document,
        new_doc: &mut Document,
        arr: &[Object],
        copied: &mut HashSet<lopdf::ObjectId>,
        id_map: &mut std::collections::HashMap<lopdf::ObjectId, lopdf::ObjectId>,
    ) -> Vec<Object> {
        arr.iter()
            .map(|val| Self::process_object_refs(doc, new_doc, val, copied, id_map))
            .collect()
    }

    /// 处理单个对象中的引用
    fn process_object_refs(
        doc: &Document,
        new_doc: &mut Document,
        obj: &Object,
        copied: &mut HashSet<lopdf::ObjectId>,
        id_map: &mut std::collections::HashMap<lopdf::ObjectId, lopdf::ObjectId>,
    ) -> Object {
        match obj {
            Object::Reference(ref_id) => {
                if let Some(new_id) = Self::copy_object_recursive(doc, new_doc, *ref_id, copied, id_map) {
                    Object::Reference(new_id)
                } else {
                    obj.clone()
                }
            }
            Object::Dictionary(dict) => {
                let new_dict = Self::process_dict_refs(doc, new_doc, &mut dict.clone(), copied, id_map);
                Object::Dictionary(new_dict)
            }
            Object::Array(arr) => {
                let new_arr = Self::process_array_refs(doc, new_doc, arr, copied, id_map);
                Object::Array(new_arr)
            }
            other => other.clone(),
        }
    }

    /// 从源文档中提取指定页面，构建新的完整PDF
    fn extract_pages_to_new_doc(doc: &Document, page_indices: &[u32]) -> Result<Document, String> {
        let page_objects = doc.get_pages();
        let mut new_doc = Document::with_version("1.5");
        let mut copied: HashSet<lopdf::ObjectId> = HashSet::new();
        let mut id_map: std::collections::HashMap<lopdf::ObjectId, lopdf::ObjectId> = std::collections::HashMap::new();

        let mut kids: Vec<Object> = Vec::new();

        for &page_idx in page_indices {
            if let Some(&page_ref) = page_objects.get(&page_idx) {
                if let Some(new_page_id) = Self::copy_object_recursive(doc, &mut new_doc, page_ref, &mut copied, &mut id_map) {
                    kids.push(Object::Reference(new_page_id));
                }
            }
        }

        // 创建 Pages 字典
        let pages_dict = Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(kids.clone())),
            ("Count", Object::Integer(kids.len() as i64)),
        ]);
        let pages_ref = new_doc.add_object(Object::Dictionary(pages_dict));

        // 创建 Catalog
        let catalog_dict = Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_ref)),
        ]);
        let catalog_ref = new_doc.add_object(Object::Dictionary(catalog_dict));

        new_doc.trailer = lopdf::Dictionary::from_iter(vec![
            ("Root", Object::Reference(catalog_ref)),
        ]);

        Ok(new_doc)
    }

    /// 拆分PDF - 按页码范围
    pub fn split_by_range(
        input_path: &Path,
        output_dir: &Path,
        ranges: &str,
    ) -> Result<Vec<String>, String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);
        let pages = PdfUtils::parse_page_range(ranges, total_pages)?;

        let mut output_files = Vec::new();
        let input_name = input_path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("output");

        // 按连续范围分组
        let mut groups: Vec<Vec<u32>> = Vec::new();
        let mut current_group: Vec<u32> = Vec::new();

        for &page in &pages {
            if current_group.is_empty() || page == current_group.last().unwrap() + 1 {
                current_group.push(page);
            } else {
                groups.push(current_group);
                current_group = vec![page];
            }
        }
        if !current_group.is_empty() {
            groups.push(current_group);
        }

        for (i, group) in groups.iter().enumerate() {
            let output_name = if groups.len() == 1 {
                format!("{}_pages_{}-{}.pdf", input_name, group[0], group.last().unwrap())
            } else {
                format!("{}_part_{}.pdf", input_name, i + 1)
            };
            let output_path = output_dir.join(&output_name);

            // 确保输出目录存在
            if let Some(parent) = output_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }

            let new_doc = Self::extract_pages_to_new_doc(&doc, group)?;
            let mut out_doc = new_doc;
            PdfUtils::save_pdf(&mut out_doc, &output_path)?;
            output_files.push(output_path.to_string_lossy().to_string());
        }

        Ok(output_files)
    }

    /// 拆分PDF - 每N页一个文件
    pub fn split_by_interval(
        input_path: &Path,
        output_dir: &Path,
        interval: usize,
    ) -> Result<Vec<String>, String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);
        let mut output_files = Vec::new();

        for start in (1..=total_pages).step_by(interval) {
            let end = std::cmp::min(start + interval - 1, total_pages);
            let range: Vec<u32> = (start as u32..=end as u32).collect();
            let new_doc = Self::extract_pages_to_new_doc(&doc, &range)?;
            let input_name = input_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("output");
            let output_name = format!("{}_pages_{}-{}.pdf", input_name, start, end);
            let output_path = output_dir.join(&output_name);
            if let Some(parent) = output_path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let mut out_doc = new_doc;
            PdfUtils::save_pdf(&mut out_doc, &output_path)?;
            output_files.push(output_path.to_string_lossy().to_string());
        }

        Ok(output_files)
    }

    /// 合并多个PDF文件
    pub fn merge_pdfs(
        input_paths: &[&Path],
        output_path: &Path,
    ) -> Result<(), String> {
        if input_paths.is_empty() {
            return Err("没有要合并的文件".to_string());
        }

        let first_doc = PdfUtils::open_pdf(input_paths[0])?;
        let mut merged_doc = first_doc;

        for input_path in &input_paths[1..] {
            let doc = PdfUtils::open_pdf(input_path)?;
            for (key, obj) in doc.objects {
                merged_doc.objects.insert(key, obj);
            }
        }

        PdfUtils::save_pdf(&mut merged_doc, output_path)
    }

    /// 旋转PDF页面
    pub fn rotate_pages(
        input_path: &Path,
        output_path: &Path,
        pages: &str,
        angle: u32,
    ) -> Result<(), String> {
        let mut doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);
        let page_list = PdfUtils::parse_page_range(pages, total_pages)?;
        let page_set: std::collections::HashSet<u32> = page_list.into_iter().collect();

        let page_objects = doc.get_pages();

        for (&page_num, &page_ref) in &page_objects {
            if page_set.contains(&(page_num as u32)) {
                if let Ok(obj) = doc.get_object(page_ref).cloned() {
                    if let Object::Dictionary(mut dict) = obj {
                        let current_rotation = match dict.get(b"Rotate") {
                            Ok(Object::Integer(r)) => *r as u32,
                            _ => 0,
                        };
                        let new_rotation = (current_rotation + angle) % 360;
                        let _ = dict.set("Rotate", Object::Integer(new_rotation as i64));
                        let _ = doc.objects.insert(page_ref, Object::Dictionary(dict));
                    }
                }
            }
        }

        PdfUtils::save_pdf(&mut doc, output_path)
    }

    /// 删除PDF页面
    pub fn delete_pages(
        input_path: &Path,
        output_path: &Path,
        pages: &str,
    ) -> Result<(), String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);
        let pages_to_delete = PdfUtils::parse_page_range(pages, total_pages)?;
        let pages_set: std::collections::HashSet<u32> = pages_to_delete.into_iter().collect();

        let keep_pages: Vec<u32> = (1..=total_pages as u32)
            .filter(|p| !pages_set.contains(p))
            .collect();

        let new_doc = Self::extract_pages_to_new_doc(&doc, &keep_pages)?;
        let mut out_doc = new_doc;
        PdfUtils::save_pdf(&mut out_doc, output_path)
    }

    /// 提取PDF页面
    pub fn extract_pages(
        input_path: &Path,
        output_path: &Path,
        pages: &str,
    ) -> Result<(), String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);
        let pages_to_extract = PdfUtils::parse_page_range(pages, total_pages)?;

        let new_doc = Self::extract_pages_to_new_doc(&doc, &pages_to_extract)?;
        let mut out_doc = new_doc;
        PdfUtils::save_pdf(&mut out_doc, output_path)
    }
}
