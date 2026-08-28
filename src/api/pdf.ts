import { invoke } from '@tauri-apps/api/core'

export interface PdfInfo {
  page_count: number
  file_size: number
  file_name: string
}

/**
 * 拆分PDF - 按页码范围
 */
export async function splitPdf(
  inputPath: string,
  outputDir: string,
  ranges: string
): Promise<string[]> {
  return invoke<string[]>('split_pdf', {
    inputPath,
    outputDir,
    ranges
  })
}

/**
 * 拆分PDF - 每N页一个文件
 */
export async function splitPdfByInterval(
  inputPath: string,
  outputDir: string,
  interval: number
): Promise<string[]> {
  return invoke<string[]>('split_pdf_by_interval', {
    inputPath,
    outputDir,
    interval
  })
}

/**
 * 合并多个PDF文件
 */
export async function mergePdfs(
  inputPaths: string[],
  outputPath: string
): Promise<void> {
  return invoke('merge_pdfs', {
    inputPaths,
    outputPath
  })
}

/**
 * 旋转PDF页面
 */
export async function rotatePdfPages(
  inputPath: string,
  outputPath: string,
  pages: string,
  angle: number
): Promise<void> {
  return invoke('rotate_pdf_pages', {
    inputPath,
    outputPath,
    pages,
    angle
  })
}

/**
 * 删除PDF页面
 */
export async function deletePdfPages(
  inputPath: string,
  outputPath: string,
  pages: string
): Promise<void> {
  return invoke('delete_pdf_pages', {
    inputPath,
    outputPath,
    pages
  })
}

/**
 * 提取PDF页面
 */
export async function extractPdfPages(
  inputPath: string,
  outputPath: string,
  pages: string
): Promise<void> {
  return invoke('extract_pdf_pages', {
    inputPath,
    outputPath,
    pages
  })
}

/**
 * 获取PDF信息
 */
export async function getPdfInfo(path: string): Promise<PdfInfo> {
  return invoke<PdfInfo>('get_pdf_info', { path })
}

/**
 * 图片转PDF
 */
export async function imagesToPdf(
  imagePaths: string[],
  outputPath: string
): Promise<void> {
  return invoke('images_to_pdf', {
    imagePaths,
    outputPath
  })
}

/**
 * PDF转文本
 */
export async function pdfToText(
  inputPath: string,
  outputPath?: string
): Promise<string> {
  return invoke<string>('pdf_to_text', {
    inputPath,
    outputPath
  })
}

/**
 * 添加水印
 */
export async function addWatermark(
  inputPath: string,
  outputPath: string,
  text: string,
  opacity: number,
  rotation: number,
  fontSize: number,
  watermarkType: string,
  imagePath?: string,
  color: string = '0,0,0',
  density: string = 'single',
  position: string = 'center'
): Promise<string> {
  return invoke<string>('add_watermark', {
    inputPath,
    outputPath,
    text,
    opacity,
    rotation,
    fontSize,
    watermarkType,
    imagePath: imagePath || null,
    color,
    density,
    position
  })
}

export interface CompressResult {
  original_size: number
  compressed_size: number
  ratio: number
}

/**
 * 压缩PDF
 */
export async function compressPdf(
  inputPath: string,
  outputPath: string,
  level: 'light' | 'medium' | 'high'
): Promise<CompressResult> {
  return invoke<CompressResult>('compress_pdf', {
    inputPath,
    outputPath,
    level
  })
}

/**
 * 加密PDF
 */
export async function encryptPdf(
  inputPath: string,
  outputPath: string,
  userPassword: string,
  ownerPassword: string,
  permissions: string[]
): Promise<void> {
  return invoke('encrypt_pdf', {
    inputPath,
    outputPath,
    userPassword,
    ownerPassword,
    permissions
  })
}

/**
 * 解密PDF
 */
export async function decryptPdf(
  inputPath: string,
  outputPath: string,
  password: string
): Promise<void> {
  return invoke('decrypt_pdf', {
    inputPath,
    outputPath,
    password
  })
}

export interface PdfMetadata {
  title: string
  author: string
  subject: string
  keywords: string
}

/**
 * 获取PDF元数据
 */
export async function getPdfMetadata(path: string): Promise<PdfMetadata> {
  return invoke<PdfMetadata>('get_pdf_metadata', { path })
}

/**
 * 更新PDF元数据
 */
export async function updatePdfMetadata(
  inputPath: string,
  outputPath: string,
  metadata: PdfMetadata
): Promise<void> {
  return invoke('update_pdf_metadata', {
    inputPath,
    outputPath,
    metadata
  })
}

/**
 * 清空PDF元数据
 */
export async function clearPdfMetadata(
  inputPath: string,
  outputPath: string
): Promise<void> {
  return invoke('clear_pdf_metadata', {
    inputPath,
    outputPath
  })
}

/**
 * 检查Tesseract是否安装
 */
export async function checkTesseract(): Promise<boolean> {
  return invoke<boolean>('check_tesseract')
}

/**
 * OCR识别图片
 */
export async function ocrImage(
  inputPath: string,
  outputPath?: string,
  language: string = 'chi_sim'
): Promise<string> {
  return invoke<string>('ocr_image', {
    inputPath,
    outputPath,
    language
  })
}

/**
 * 读取文件为base64
 */
export async function readFileAsBase64(path: string): Promise<string> {
  return invoke<string>('read_file_as_base64', { path })
}

export async function copyFile(src: string, dst: string): Promise<void> {
  return invoke('copy_file', { src, dst })
}

export async function openFile(path: string): Promise<void> {
  return invoke('open_file', { path })
}

export async function openFolder(path: string): Promise<void> {
  return invoke('open_folder', { path })
}
