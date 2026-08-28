# 快PDF - 本地PDF处理工具

一个基于 Tauri 2 + Vue 3 的桌面PDF处理工具，支持Windows、Linux、macOS。

## 特性

- **隐私友好**：所有处理都在本地完成，文件不上传
- **轻量级**：无浏览器臃肿，原生桌面应用
- **跨平台**：支持 Windows、Linux、macOS
- **功能丰富**：涵盖PDF处理的各个方面

## 功能模块

### 一、页面操作
- PDF 拆分（按页码范围、每N页）
- PDF 合并（拖拽排序）
- 页面旋转（90/180/270度）
- 页面删除
- 页面提取
- 页面重排

### 二、格式转换
- 图片转 PDF
- PDF 转文本

### 三、PDF 编辑
- 添加水印（文字水印）
- 压缩 PDF（轻量/中等/高压缩）

### 四、安全与权限
- PDF 加密（打开密码、权限密码）
- PDF 解密
- 元数据修改
- 元数据清空

### 五、OCR 识别
- OCR 识别图片文字（需要安装 Tesseract）

### 六、批量处理
- 批量压缩
- 批量加密

### 七、高级功能
- 查看 PDF 信息

### 八、工具
- 任务队列
- 操作日志

## 开发环境要求

- Node.js 18+
- Rust 1.77+
- Tauri CLI

## 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI（如果未安装）
cargo install tauri-cli
```

## 开发

```bash
# 启动开发服务器
npm run tauri dev
```

## 构建

```bash
# 构建生产版本
npm run tauri build
```

## 项目结构

```
kuai-pdf/
├── src/                    # Vue 前端
│   ├── api/               # API 调用
│   ├── components/        # Vue 组件
│   │   └── pdf/          # PDF 工具组件
│   ├── App.vue            # 主界面
│   └── main.ts            # 入口
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   ├── lib.rs         # Tauri 命令注册
│   │   ├── pdf_utils.rs   # PDF 工具函数
│   │   ├── page_ops.rs    # 页面操作
│   │   ├── converter.rs   # 格式转换
│   │   ├── editor.rs      # PDF 编辑
│   │   ├── security.rs    # 安全与权限
│   │   └── ocr.rs         # OCR 识别
│   ├── Cargo.toml         # Rust 依赖
│   └── tauri.conf.json    # Tauri 配置
└── package.json
```

## 注意事项

1. **OCR 功能**：需要安装 Tesseract OCR 引擎
   - Windows: 下载安装 https://github.com/UB-Mannheim/tesseract/wiki
   - macOS: `brew install tesseract`
   - Linux: `sudo apt install tesseract-ocr`

2. **图片转 PDF**：支持 PNG、JPG、BMP 格式

3. **加密功能**：使用 PDF 标准加密算法

## 技术栈

- **前端**：Vue 3 + TypeScript + Element Plus
- **后端**：Rust + Tauri 2
- **构建**：Vite

## 许可证

MIT License
