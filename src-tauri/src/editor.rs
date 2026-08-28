use lopdf::{Object, Dictionary};
use lopdf::content::{Operation, Content};
use std::path::Path;
use image::{Rgba, RgbaImage, GenericImageView};
use rusttype::{Font, Scale};
use crate::pdf_utils::PdfUtils;

pub struct Editor;

impl Editor {
    /// 加载系统字体
    fn load_system_font() -> Result<Vec<u8>, String> {
        // Windows 系统字体路径
        let font_paths = if cfg!(target_os = "windows") {
            vec![
                "C:\\Windows\\Fonts\\msyh.ttc",    // 微软雅黑
                "C:\\Windows\\Fonts\\msyhbd.ttc",   // 微软雅黑粗体
                "C:\\Windows\\Fonts\\simsun.ttc",   // 宋体
                "C:\\Windows\\Fonts\\simhei.ttf",   // 黑体
                "C:\\Windows\\Fonts\\arial.ttf",     // Arial (英文)
            ]
        } else if cfg!(target_os = "macos") {
            vec![
                "/System/Library/Fonts/PingFang.ttc",
                "/System/Library/Fonts/STHeiti Light.ttc",
                "/Library/Fonts/Arial.ttf",
            ]
        } else {
            vec![
                "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            ]
        };

        for path in font_paths {
            if let Ok(data) = std::fs::read(path) {
                return Ok(data);
            }
        }

        Err("无法找到系统字体".to_string())
    }

    /// 将文字渲染为 (RGB数据, Alpha数据, 宽度, 高度)
    fn render_text_to_raw(
        text: &str,
        font_size: u32,
        opacity: f32,
        r: u8, g: u8, b: u8,
    ) -> Result<(Vec<u8>, Vec<u8>, u32, u32), String> {
        let font_data = Self::load_system_font()?;
        let font = Font::try_from_vec(font_data)
            .ok_or("加载字体失败")?;

        let scale = Scale::uniform(font_size as f32);

        // 计算文字尺寸
        let glyphs: Vec<_> = font.layout(text, scale, rusttype::point(0.0, 0.0)).collect();

        // 计算文字的实际边界框
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for glyph in &glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                min_x = min_x.min(bb.min.x as f32);
                max_x = max_x.max(bb.max.x as f32);
                min_y = min_y.min(bb.min.y as f32);
                max_y = max_y.max(bb.max.y as f32);
            }
        }

        // 如果没有有效的字形（例如空字符串），使用默认尺寸
        let text_width = if min_x < max_x { max_x - min_x } else { font_size as f32 * text.len() as f32 * 0.6 };
        let text_height = if min_y < max_y { max_y - min_y } else { font_size as f32 };

        let padding = font_size as u32 / 2;
        let img_width = (text_width + padding as f32 * 2.0) as u32;
        let img_height = (text_height + padding as f32 * 2.0) as u32;

        // 创建透明图片（白色背景）
        let mut img = RgbaImage::new(img_width, img_height);

        let alpha = (opacity * 255.0) as u8;

        // 绘制文字（调整偏移量使文字居中）
        let offset_x = padding as i32 - min_x as i32;
        let offset_y = padding as i32 - min_y as i32;

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x + offset_x;
                    let y = y as i32 + bb.min.y + offset_y;

                    if x >= 0 && x < img_width as i32 && y >= 0 && y < img_height as i32 {
                        let pixel_alpha = (v * alpha as f32) as u8;
                        let pixel = Rgba([r, g, b, pixel_alpha]);
                        img.put_pixel(x as u32, y as u32, pixel);
                    }
                });
            }
        }

        // 提取 RGB 和 Alpha 数据（透明区域使用白色背景）
        let mut rgb_data = Vec::with_capacity((img_width * img_height * 3) as usize);
        let mut alpha_data = Vec::with_capacity((img_width * img_height) as usize);

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel = img.get_pixel(x, y);
                let a = pixel[3];
                if a > 0 {
                    // 有内容的像素：使用文字颜色
                    rgb_data.push(pixel[0]);
                    rgb_data.push(pixel[1]);
                    rgb_data.push(pixel[2]);
                } else {
                    // 透明像素：使用白色背景（避免黑色底图）
                    rgb_data.push(255);
                    rgb_data.push(255);
                    rgb_data.push(255);
                }
                alpha_data.push(a);
            }
        }

        Ok((rgb_data, alpha_data, img_width, img_height))
    }

    pub fn add_watermark(
        input_path: &Path,
        _output_path: &Path,
        text: &str,
        opacity: f32,
        rotation: f32,
        font_size: u32,
        watermark_type: &str,
        image_path: Option<&Path>,
        color: &str,
        density: &str,
        position: &str,
    ) -> Result<String, String> {
        // 使用系统临时目录，确保有写权限
        let temp_dir = std::env::temp_dir();
        let file_name = input_path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let actual_output = temp_dir.join(format!("{}_watermarked_{}.pdf", file_name, ts));

        // 复制原始文件到临时目录
        std::fs::copy(input_path, &actual_output)
            .map_err(|e| format!("复制文件失败: {}", e))?;

        let mut doc = PdfUtils::open_pdf(&actual_output)?;
        let page_objects = doc.get_pages();

        let (r_u8, g_u8, b_u8): (u8, u8, u8) = {
            let p: Vec<f32> = color.split(',').filter_map(|s| s.trim().parse().ok()).collect();
            if p.len() >= 3 {
                ((p[0] * 255.0) as u8, (p[1] * 255.0) as u8, (p[2] * 255.0) as u8)
            } else {
                (0, 0, 0)
            }
        };

        // 准备水印图片数据（RGB + Alpha 分离）
        let (img_data, alpha_data, img_w, img_h) = if watermark_type == "text" {
            // 文字水印：直接渲染为原始数据
            Self::render_text_to_raw(text, font_size, opacity, r_u8, g_u8, b_u8)?
        } else if let Some(img_path) = image_path {
            // 图片水印：读取并处理
            let img = image::open(img_path)
                .map_err(|e| format!("打开水印图片失败: {}", e))?;

            let (w, h) = img.dimensions();
            let rgba_img = img.to_rgba8();

            let mut rgb_data = Vec::with_capacity((w * h * 3) as usize);
            let mut alpha_vec = Vec::with_capacity((w * h) as usize);

            for y in 0..h {
                for x in 0..w {
                    let pixel = rgba_img.get_pixel(x, y);
                    rgb_data.push(pixel[0]);
                    rgb_data.push(pixel[1]);
                    rgb_data.push(pixel[2]);
                    // 应用透明度
                    alpha_vec.push((pixel[3] as f32 * opacity) as u8);
                }
            }

            (rgb_data, alpha_vec, w, h)
        } else {
            return Err("未指定水印图片路径".to_string());
        };

        // 转换为 PDF 单位（72 DPI，假设图片是 96 DPI 渲染的）
        let pdf_img_w = img_w as f32 * 72.0 / 96.0;
        let pdf_img_h = img_h as f32 * 72.0 / 96.0;

        // 压缩数据
        let compressed_rgb = compress_raw_data(&img_data);
        let compressed_alpha = compress_raw_data(&alpha_data);

        // 创建 ExtGState
        let mut gs_dict = Dictionary::new();
        let _ = gs_dict.set("CA", Object::Real(1.0));
        let _ = gs_dict.set("ca", Object::Real(1.0));
        let gs_ref = doc.add_object(Object::Dictionary(gs_dict));

        // 创建图片对象
        let mut img_dict = Dictionary::new();
        let _ = img_dict.set("Type", Object::Name(b"XObject".to_vec()));
        let _ = img_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        let _ = img_dict.set("Width", Object::Integer(img_w as i64));
        let _ = img_dict.set("Height", Object::Integer(img_h as i64));
        let _ = img_dict.set("ColorSpace", Object::Name(b"DeviceRGB".to_vec()));
        let _ = img_dict.set("BitsPerComponent", Object::Integer(8));
        let _ = img_dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));

        // 创建 Alpha 通道对象
        let mut alpha_dict = Dictionary::new();
        let _ = alpha_dict.set("Type", Object::Name(b"XObject".to_vec()));
        let _ = alpha_dict.set("Subtype", Object::Name(b"Image".to_vec()));
        let _ = alpha_dict.set("Width", Object::Integer(img_w as i64));
        let _ = alpha_dict.set("Height", Object::Integer(img_h as i64));
        let _ = alpha_dict.set("ColorSpace", Object::Name(b"DeviceGray".to_vec()));
        let _ = alpha_dict.set("BitsPerComponent", Object::Integer(8));
        let _ = alpha_dict.set("Filter", Object::Name(b"FlateDecode".to_vec()));

        let alpha_ref = doc.add_object(Object::Stream(lopdf::Stream::new(alpha_dict, compressed_alpha)));
        let _ = img_dict.set("SMask", Object::Reference(alpha_ref));

        let img_ref = doc.add_object(Object::Stream(lopdf::Stream::new(img_dict, compressed_rgb)));

        for (&_page_num, &page_ref) in &page_objects {
            // 获取页面尺寸
            let (width, height) = {
                let pd = doc.get_dictionary(page_ref).map_err(|e| format!("读取页面失败: {}", e))?;
                let w = pd.get(b"MediaBox").ok().and_then(|v| if let Object::Array(a) = v {
                    a.get(2).and_then(|x| if let Object::Integer(i) = x { Some(*i as f32) } else { None })
                } else { None }).unwrap_or(595.0);
                let h = pd.get(b"MediaBox").ok().and_then(|v| if let Object::Array(a) = v {
                    a.get(3).and_then(|x| if let Object::Integer(i) = x { Some(*i as f32) } else { None })
                } else { None }).unwrap_or(842.0);
                (w, h)
            };

            // === 添加图片到 Resources ===
            {
                let page_mut = doc.get_dictionary_mut(page_ref)
                    .map_err(|e| format!("获取页面可变引用失败: {}", e))?;

                if page_mut.get(b"Resources").is_err() {
                    let _ = page_mut.set("Resources", Object::Dictionary(Dictionary::new()));
                }

                let res_ref = page_mut.get(b"Resources")
                    .and_then(|v| v.as_reference())
                    .ok();

                if let Some(rr) = res_ref {
                    // 获取 ExtGState 的引用 ID
                    let ext_gs_ref_id = {
                        let res_mut = doc.get_dictionary_mut(rr)
                            .map_err(|e| format!("获取Resources可变引用失败: {}", e))?;

                        if let Ok(existing) = res_mut.get(b"ExtGState") {
                            if let Object::Reference(r) = existing {
                                Some(*r)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    // 修改 ExtGState
                    if let Some(ref_id) = ext_gs_ref_id {
                        if let Ok(ext_gs) = doc.get_dictionary_mut(ref_id) {
                            let _ = ext_gs.set("GS0", Object::Reference(gs_ref));
                        }
                    } else {
                        let mut ext_gs = Dictionary::new();
                        let _ = ext_gs.set("GS0", Object::Reference(gs_ref));
                        let ext_gs_ref = doc.add_object(Object::Dictionary(ext_gs));
                        let res_mut = doc.get_dictionary_mut(rr)
                            .map_err(|e| format!("获取Resources可变引用失败: {}", e))?;
                        let _ = res_mut.set("ExtGState", Object::Reference(ext_gs_ref));
                    }

                    // 获取 XObject 的引用 ID
                    let xobjects_ref_id = {
                        let res_mut = doc.get_dictionary_mut(rr)
                            .map_err(|e| format!("获取Resources可变引用失败: {}", e))?;

                        if let Ok(existing) = res_mut.get(b"XObject") {
                            if let Object::Reference(r) = existing {
                                Some(*r)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    // 修改 XObject
                    if let Some(ref_id) = xobjects_ref_id {
                        if let Ok(xobjects) = doc.get_dictionary_mut(ref_id) {
                            let _ = xobjects.set("Watermark", Object::Reference(img_ref));
                        }
                    } else {
                        let mut xobjects = Dictionary::new();
                        let _ = xobjects.set("Watermark", Object::Reference(img_ref));
                        let xobjects_ref = doc.add_object(Object::Dictionary(xobjects));
                        let res_mut = doc.get_dictionary_mut(rr)
                            .map_err(|e| format!("获取Resources可变引用失败: {}", e))?;
                        let _ = res_mut.set("XObject", Object::Reference(xobjects_ref));
                    }
                }
            }

            // === 构建水印内容 ===
            let positions = get_positions(density, position, width, height, pdf_img_h);
            let mut ops: Vec<Operation> = Vec::new();

            for (px, py) in &positions {
                let rad = rotation.to_radians();
                let (cos_v, sin_v) = (rad.cos(), rad.sin());

                ops.push(Operation::new("q", vec![]));

                // 应用透明度
                ops.push(Operation::new("gs", vec![
                    Object::Name(b"GS0".to_vec()),
                ]));

                // 应用变换矩阵：缩放 + 旋转 + 平移
                let (tx, ty) = if rotation != 0.0 {
                    // 计算旋转后的偏移
                    let cx = pdf_img_w / 2.0;
                    let cy = pdf_img_h / 2.0;
                    let new_cx = cx * cos_v + cy * sin_v;
                    let new_cy = -cx * sin_v + cy * cos_v;
                    (*px + cx - new_cx, *py + cy - new_cy)
                } else {
                    (*px, *py)
                };

                ops.push(Operation::new("cm", vec![
                    Object::Real(pdf_img_w * cos_v), Object::Real(pdf_img_w * sin_v),
                    Object::Real(-pdf_img_h * sin_v), Object::Real(pdf_img_h * cos_v),
                    Object::Real(tx), Object::Real(ty),
                ]));

                // 绘制图片
                ops.push(Operation::new("Do", vec![
                    Object::Name(b"Watermark".to_vec()),
                ]));

                ops.push(Operation::new("Q", vec![]));
            }

            // 追加到页面内容
            let content = Content { operations: ops };
            let _ = doc.add_to_page_content(page_ref, content);
        }

        // 保存到输出路径
        PdfUtils::save_pdf(&mut doc, &actual_output)?;
        Ok(actual_output.to_string_lossy().to_string())
    }

    pub fn compress_pdf(
        input_path: &Path, output_path: &Path, _level: &str,
    ) -> Result<serde_json::Value, String> {
        let orig = std::fs::metadata(input_path).map(|m| m.len()).unwrap_or(0);
        let mut doc = PdfUtils::open_pdf(input_path)?;
        PdfUtils::save_pdf(&mut doc, output_path)?;
        let comp = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
        let ratio = if orig > 0 { ((orig as f64 - comp as f64) / orig as f64 * 100.0) as u32 } else { 0 };
        Ok(serde_json::json!({ "original_size": orig, "compressed_size": comp, "ratio": ratio }))
    }
}

/// 压缩原始数据
fn compress_raw_data(data: &[u8]) -> Vec<u8> {
    use flate2::write::ZlibEncoder;
    use flate2::Compression;
    use std::io::Write;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    let _ = encoder.write_all(data);
    encoder.finish().unwrap_or_default()
}

fn get_positions(density: &str, position: &str, width: f32, height: f32, font_size: f32) -> Vec<(f32, f32)> {
    let mut pos = Vec::new();
    match density {
        "row" => {
            let step = font_size * 3.0;
            let mut x = 0.0;
            while x < width + step { pos.push((x, height / 2.0)); x += step; }
        }
        "grid" => {
            let (sx, sy) = (font_size * 4.0, font_size * 4.0);
            let mut y = 0.0;
            while y < height + sy { let mut x = 0.0; while x < width + sx { pos.push((x, y)); x += sx; } y += sy; }
        }
        _ => {
            pos.push(match position {
                "top-left" => (width * 0.05, height * 0.9),
                "top-right" => (width * 0.55, height * 0.9),
                "bottom-left" => (width * 0.05, height * 0.05),
                "bottom-right" => (width * 0.55, height * 0.05),
                _ => (width / 2.0, height / 2.0),
            });
        }
    }
    pos
}
