use lopdf::Document;
use std::path::Path;

pub struct PdfUtils;

impl PdfUtils {
    pub fn open_pdf(path: &Path) -> Result<Document, String> {
        Document::load(path).map_err(|e| format!("无法打开PDF文件: {}", e))
    }

    pub fn save_pdf(doc: &mut Document, path: &Path) -> Result<(), String> {
        // 尝试使用更安全的保存方式，避免重新编码导致乱码
        doc.save(path)
            .map_err(|e| format!("无法保存PDF文件: {}", e))?;

        // 验证保存后的文件
        if !path.exists() {
            return Err("保存后文件不存在".to_string());
        }

        let file_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(0);

        if file_size == 0 {
            return Err("保存后的文件为空".to_string());
        }

        Ok(())
    }

    pub fn get_page_count(doc: &Document) -> usize {
        doc.get_pages().len()
    }

    pub fn parse_page_range(range_str: &str, total_pages: usize) -> Result<Vec<u32>, String> {
        let mut pages = Vec::new();
        let parts: Vec<&str> = range_str.split(',').collect();

        for part in parts {
            let part = part.trim();
            if part.contains('-') {
                let range_parts: Vec<&str> = part.split('-').collect();
                if range_parts.len() != 2 {
                    return Err(format!("无效的页码范围: {}", part));
                }
                let start: u32 = range_parts[0].trim().parse()
                    .map_err(|_| format!("无效的页码: {}", range_parts[0]))?;
                let end: u32 = range_parts[1].trim().parse()
                    .map_err(|_| format!("无效的页码: {}", range_parts[1]))?;

                if start < 1 || end > total_pages as u32 || start > end {
                    return Err(format!("页码范围超出有效范围: {}", part));
                }

                for i in start..=end {
                    pages.push(i);
                }
            } else {
                let page: u32 = part.parse()
                    .map_err(|_| format!("无效的页码: {}", part))?;

                if page < 1 || page > total_pages as u32 {
                    return Err(format!("页码超出有效范围: {}", page));
                }

                pages.push(page);
            }
        }

        Ok(pages)
    }
}
