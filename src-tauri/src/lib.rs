mod pdf_utils;
mod page_ops;
mod converter;
mod editor;
mod security;
mod ocr;

use std::path::PathBuf;
use tauri::command;
use base64::Engine;

#[command]
fn read_file_as_base64(path: String) -> Result<String, String> {
    let data = std::fs::read(&path).map_err(|e| format!("读取文件失败: {}", e))?;
    Ok(base64::engine::general_purpose::STANDARD.encode(&data))
}

#[command]
fn copy_file(src: String, dst: String) -> Result<(), String> {
    std::fs::copy(&src, &dst).map_err(|e| format!("复制文件失败: {}", e))?;
    Ok(())
}

#[command]
fn open_file(path: String) -> Result<(), String> {
    open::that(&path).map_err(|e| format!("打开文件失败: {}", e))
}

#[command]
fn open_folder(path: String) -> Result<(), String> {
    // 打开文件所在目录
    let parent = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| path.clone());
    open::that(&parent).map_err(|e| format!("打开目录失败: {}", e))
}

// ==================== 页面操作 ====================

#[command]
fn split_pdf(input_path: String, output_dir: String, ranges: String) -> Result<Vec<String>, String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_dir);
    page_ops::PageOps::split_by_range(&input, &output, &ranges)
}

#[command]
fn split_pdf_by_interval(input_path: String, output_dir: String, interval: usize) -> Result<Vec<String>, String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_dir);
    page_ops::PageOps::split_by_interval(&input, &output, interval)
}

#[command]
fn merge_pdfs(input_paths: Vec<String>, output_path: String) -> Result<(), String> {
    let inputs: Vec<PathBuf> = input_paths.iter().map(PathBuf::from).collect();
    let input_refs: Vec<&std::path::Path> = inputs.iter().map(|p| p.as_path()).collect();
    let output = PathBuf::from(&output_path);
    page_ops::PageOps::merge_pdfs(&input_refs, &output)
}

#[command]
fn rotate_pdf_pages(input_path: String, output_path: String, pages: String, angle: u32) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    page_ops::PageOps::rotate_pages(&input, &output, &pages, angle)
}

#[command]
fn delete_pdf_pages(input_path: String, output_path: String, pages: String) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    page_ops::PageOps::delete_pages(&input, &output, &pages)
}

#[command]
fn extract_pdf_pages(input_path: String, output_path: String, pages: String) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    page_ops::PageOps::extract_pages(&input, &output, &pages)
}

// ==================== 信息 ====================

#[command]
fn get_pdf_info(path: String) -> Result<serde_json::Value, String> {
    let file_path = PathBuf::from(&path);
    let doc = pdf_utils::PdfUtils::open_pdf(&file_path)?;
    let page_count = pdf_utils::PdfUtils::get_page_count(&doc);
    let file_size = std::fs::metadata(&file_path).map(|m| m.len()).unwrap_or(0);
    let file_name = file_path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_string();

    Ok(serde_json::json!({
        "page_count": page_count,
        "file_size": file_size,
        "file_name": file_name,
    }))
}

// ==================== 格式转换 ====================

#[command]
fn images_to_pdf(image_paths: Vec<String>, output_path: String) -> Result<(), String> {
    let inputs: Vec<PathBuf> = image_paths.iter().map(PathBuf::from).collect();
    let input_refs: Vec<&std::path::Path> = inputs.iter().map(|p| p.as_path()).collect();
    let output = PathBuf::from(&output_path);
    converter::Converter::images_to_pdf(&input_refs, &output)
}

#[command]
fn pdf_to_text(input_path: String, output_path: Option<String>) -> Result<String, String> {
    let input = PathBuf::from(&input_path);
    let output = output_path.map(PathBuf::from);
    converter::Converter::pdf_to_text(&input, output.as_deref())
}

// ==================== 编辑 ====================

#[command]
fn compress_pdf(input_path: String, output_path: String, level: String) -> Result<serde_json::Value, String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    editor::Editor::compress_pdf(&input, &output, &level)
}

#[command]
fn add_watermark(
    input_path: String,
    output_path: String,
    text: String,
    opacity: f32,
    rotation: f32,
    font_size: u32,
    watermark_type: String,
    image_path: Option<String>,
    color: String,
    density: String,
    position: String,
) -> Result<String, String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    let img = image_path.as_deref().map(PathBuf::from);
    // 返回实际的输出路径
    editor::Editor::add_watermark(
        &input, &output, &text, opacity, rotation, font_size,
        &watermark_type, img.as_deref(), &color, &density, &position,
    )
}

// ==================== 安全 ====================

#[command]
fn encrypt_pdf(
    input_path: String,
    output_path: String,
    user_password: String,
    owner_password: String,
    permissions: Vec<String>,
) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    let perms: Vec<&str> = permissions.iter().map(|s| s.as_str()).collect();
    security::Security::encrypt_pdf(&input, &output, &user_password, &owner_password, &perms)
}

#[command]
fn decrypt_pdf(input_path: String, output_path: String, password: String) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    security::Security::decrypt_pdf(&input, &output, &password)
}

#[command]
fn get_pdf_metadata(path: String) -> Result<serde_json::Value, String> {
    let file_path = PathBuf::from(&path);
    security::Security::get_metadata(&file_path)
}

#[command]
fn update_pdf_metadata(input_path: String, output_path: String, metadata: serde_json::Value) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    security::Security::update_metadata(&input, &output, &metadata)
}

#[command]
fn clear_pdf_metadata(input_path: String, output_path: String) -> Result<(), String> {
    let input = PathBuf::from(&input_path);
    let output = PathBuf::from(&output_path);
    security::Security::clear_metadata(&input, &output)
}

// ==================== OCR ====================

#[command]
fn check_tesseract() -> Result<bool, String> {
    ocr::Ocr::check_tesseract()
}

#[command]
fn ocr_image(input_path: String, output_path: Option<String>, language: String) -> Result<String, String> {
    let input = PathBuf::from(&input_path);
    let output = output_path.map(PathBuf::from);
    ocr::Ocr::ocr_image(&input, output.as_deref(), &language)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            split_pdf,
            split_pdf_by_interval,
            merge_pdfs,
            rotate_pdf_pages,
            delete_pdf_pages,
            extract_pdf_pages,
            get_pdf_info,
            images_to_pdf,
            pdf_to_text,
            compress_pdf,
            add_watermark,
            encrypt_pdf,
            decrypt_pdf,
            get_pdf_metadata,
            update_pdf_metadata,
            clear_pdf_metadata,
            check_tesseract,
            ocr_image,
            read_file_as_base64,
            copy_file,
            open_file,
            open_folder,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
