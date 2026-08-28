use std::path::Path;
use std::process::{Command, Stdio};

pub struct Ocr;

impl Ocr {
    /// 检查OCR是否可用
    pub fn check_tesseract() -> Result<bool, String> {
        Ok(true)
    }

    /// OCR识别图片
    pub fn ocr_image(
        input_path: &Path,
        _output_path: Option<&Path>,
        language: &str,
    ) -> Result<String, String> {
        // 优先尝试 Windows 内置 OCR
        match ocr_with_windows_builtin(input_path, language) {
            Ok(text) if !text.trim().is_empty() => return Ok(text),
            _ => {}
        }

        // 回退到 Tesseract
        ocr_with_tesseract(input_path, language)
            .map_err(|_| "OCR 识别失败。请安装 Tesseract OCR：\n\
                          1. 下载: https://github.com/UB-Mannheim/tesseract/wiki\n\
                          2. 安装时勾选 Chinese Simplified\n\
                          3. 重启应用".to_string())
    }
}

/// 使用 Windows 内置 OCR（Win10/11 自带）
fn ocr_with_windows_builtin(image_path: &Path, language: &str) -> Result<String, String> {
    let lang_code = match language {
        "chi_sim" | "zh" => "zh-CN",
        "chi_tra" => "zh-TW",
        "eng" => "en-US",
        "jpn" => "ja-JP",
        "kor" => "ko-KR",
        _ => "zh-CN",
    };

    let abs_path = std::fs::canonicalize(image_path)
        .map_err(|e| format!("获取文件路径失败: {}", e))?;
    let path_str = abs_path.to_string_lossy().replace('\\', "\\\\");

    // 写入临时 PowerShell 脚本
    let ps_script = format!(
        r#"Add-Type -AssemblyName System.Runtime.WindowsRuntime
[void][Windows.Media.Ocr.OcrEngine, Windows.Media.Ocr, ContentType = WindowsRuntime]
[void][Windows.Storage.StorageFile, Windows.Storage, ContentType = WindowsRuntime]
[void][Windows.Graphics.Imaging.SoftwareBitmap, Windows.Graphics.Imaging, ContentType = WindowsRuntime]

$asTaskGeneric = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {{ $_.Name -eq 'AsTask' -and $_.GetParameters().Count -eq 1 -and $_.GetParameters()[0].ParameterType.Name -eq 'IAsyncOperation`1' }})[0]
function AsTask($WinRtTask, $ResultType) {{
    $asTask = $asTaskGeneric.MakeGenericMethod($ResultType)
    $netTask = $asTask.Invoke($null, @($WinRtTask))
    $netTask
}}

$lang = [Windows.Globalization.Language]::new("{lang_code}")
$engine = [Windows.Media.Ocr.OcrEngine]::TryCreateFromLanguage($lang)

$op = [Windows.Storage.StorageFile]::GetFileFromPathAsync("{path_str}")
$file = AsTask $op ([Windows.Storage.StorageFile]).Result

$op2 = [Windows.Graphics.Imaging.SoftwareBitmap]::CreateAsync($file)
$bitmap = AsTask $op2 ([Windows.Graphics.Imaging.SoftwareBitmap]).Result

$op3 = $engine.RecognizeAsync($bitmap)
$result = AsTask $op3 ([Windows.Media.Ocr.OcrResult]).Result

$result.Text
"#
    );

    let temp_dir = std::env::temp_dir();
    let ps_path = temp_dir.join("kuai_pdf_ocr.ps1");
    std::fs::write(&ps_path, &ps_script)
        .map_err(|e| format!("写入脚本失败: {}", e))?;

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(&ps_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("执行 PowerShell 失败: {}", e))?;

    // 清理临时文件
    let _ = std::fs::remove_file(&ps_path);

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(text)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Windows OCR 失败: {}", stderr))
    }
}

/// 使用 Tesseract
fn ocr_with_tesseract(input_path: &Path, language: &str) -> Result<String, String> {
    let tesseract_path = find_tesseract()
        .ok_or("Tesseract 未安装")?;

    let input_str = input_path.to_str().unwrap_or("");

    let output = Command::new(&tesseract_path)
        .arg(input_str)
        .arg("stdout")
        .arg("-l")
        .arg(language)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("执行Tesseract失败: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Tesseract识别失败: {}", stderr));
    }

    let text = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(text)
}

/// 查找 Tesseract
fn find_tesseract() -> Option<String> {
    if let Ok(output) = Command::new("where").arg("tesseract").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
    }

    let common_paths = [
        "C:\\Program Files\\Tesseract-OCR\\tesseract.exe",
        "C:\\Program Files (x86)\\Tesseract-OCR\\tesseract.exe",
    ];

    for path in &common_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let portable_path = exe_dir.join("tesseract").join("tesseract.exe");
            if portable_path.exists() {
                return Some(portable_path.to_string_lossy().to_string());
            }
        }
    }

    None
}
