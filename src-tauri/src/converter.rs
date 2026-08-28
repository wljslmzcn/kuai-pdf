use lopdf::{Document, Object, Dictionary, Stream};
use std::path::Path;
use std::io::Cursor;
use crate::pdf_utils::PdfUtils;

pub struct Converter;

impl Converter {
    /// 图片转PDF
    pub fn images_to_pdf(
        image_paths: &[&Path],
        output_path: &Path,
    ) -> Result<(), String> {
        if image_paths.is_empty() {
            return Err("没有要转换的图片".to_string());
        }

        let mut doc = Document::with_version("1.5");
        let mut kids: Vec<Object> = Vec::new();

        for image_path in image_paths {
            let img = image::open(image_path)
                .map_err(|e| format!("无法打开图片: {}", e))?;

            let width = img.width();
            let height = img.height();

            // 把图片编码为JPEG字节
            let mut jpeg_buf = Cursor::new(Vec::new());
            img.write_to(&mut jpeg_buf, image::ImageFormat::Jpeg)
                .map_err(|e| format!("编码图片失败: {}", e))?;
            let jpeg_bytes = jpeg_buf.into_inner();

            // 创建图片 XObject
            let image_dict = Dictionary::from_iter(vec![
                ("Type", Object::Name(b"XObject".to_vec())),
                ("Subtype", Object::Name(b"Image".to_vec())),
                ("Width", Object::Integer(width as i64)),
                ("Height", Object::Integer(height as i64)),
                ("ColorSpace", Object::Name(b"DeviceRGB".to_vec())),
                ("BitsPerComponent", Object::Integer(8)),
                ("Filter", Object::Name(b"DCTDecode".to_vec())),
            ]);
            let image_ref = doc.add_object(Stream::new(image_dict, jpeg_bytes));

            // 创建内容流：把图片画满整个页面
            let content = format!(
                "q {} 0 0 {} 0 0 cm /Im0 Do Q",
                width, height
            );
            let content_dict = Dictionary::new();
            let content_ref = doc.add_object(Stream::new(content_dict, content.into_bytes()));

            // 创建 Resources 字典
            let mut xobjects = Dictionary::new();
            let _ = xobjects.set("Im0", Object::Reference(image_ref));
            let mut resources = Dictionary::new();
            let _ = resources.set("XObject", Object::Dictionary(xobjects));

            // 创建页面
            let mut page_dict = Dictionary::from_iter(vec![
                ("Type", Object::Name(b"Page".to_vec())),
                ("MediaBox", Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(width as i64),
                    Object::Integer(height as i64),
                ])),
                ("Contents", Object::Reference(content_ref)),
                ("Resources", Object::Dictionary(resources)),
            ]);

            let page_ref = doc.add_object(Object::Dictionary(page_dict));
            kids.push(Object::Reference(page_ref));
        }

        // 创建 Pages 字典
        let pages_dict = Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(kids.clone())),
            ("Count", Object::Integer(kids.len() as i64)),
        ]);
        let pages_ref = doc.add_object(Object::Dictionary(pages_dict));

        // 创建 Catalog
        let catalog_dict = Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_ref)),
        ]);
        let catalog_ref = doc.add_object(Object::Dictionary(catalog_dict));

        doc.trailer = Dictionary::from_iter(vec![
            ("Root", Object::Reference(catalog_ref)),
        ]);

        PdfUtils::save_pdf(&mut doc, output_path)
    }

    /// 提取PDF文本内容
    pub fn pdf_to_text(
        input_path: &Path,
        output_path: Option<&Path>,
    ) -> Result<String, String> {
        let doc = PdfUtils::open_pdf(input_path)?;
        let total_pages = PdfUtils::get_page_count(&doc);

        // 收集所有页码
        let page_numbers: Vec<u32> = (1..=total_pages as u32).collect();

        // 使用 lopdf 内置的 extract_text
        let text = doc.extract_text(&page_numbers)
            .map_err(|e| format!("提取文本失败: {}", e))?;

        if let Some(output) = output_path {
            std::fs::write(output, &text)
                .map_err(|e| format!("无法写入文件: {}", e))?;
        }

        Ok(text)
    }
}
