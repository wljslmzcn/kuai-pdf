<div align="center">

# 🚀 快PDF

### 本地 PDF 处理工具

**轻量 · 高效 · 隐私安全**

[![Tauri](https://img.shields.io/badge/Tauri-2.x-blue?logo=tauri)](https://tauri.app)
[![Vue](https://img.shields.io/badge/Vue-3-green?logo=vue.js)](https://vuejs.org)
[![Rust](https://img.shields.io/badge/Rust-1.77+-orange?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[下载安装](#-安装) · [功能特性](#-功能模块) · [开发指南](#-开发) · [反馈问题](https://github.com/wljslmzcn/kuai-pdf/issues)

</div>

---

## ✨ 特性

| 特性 | 说明 |
|:---:|:---|
| 🔒 **隐私安全** | 所有处理都在本地完成，文件不上传服务器 |
| ⚡ **轻量高效** | 原生桌面应用，无浏览器臃肿 |
| 🖥️ **跨平台** | 支持 Windows、Linux、macOS |
| 🎨 **现代界面** | 简洁美观的 UI，支持暗色主题 |
| 📦 **开箱即用** | 无需安装 PDF 阅读器，内置预览 |

---

## 📋 功能模块

### 📄 页面操作

| 功能 | 说明 |
|:---|:---|
| 🔀 PDF 拆分 | 按页码范围或每N页拆分 |
| 📎 PDF 合并 | 拖拽排序，灵活合并 |
| 🔄 页面旋转 | 支持 90°/180°/270° 旋转 |
| 🗑️ 页面删除 | 批量删除不需要的页面 |
| 📑 页面提取 | 提取指定页面生成新文件 |
| 📃 页面重排 | 自定义页面顺序 |

### 🔄 格式转换

| 功能 | 说明 |
|:---|:---|
| 🖼️ 图片转 PDF | 支持 PNG、JPG、BMP 格式 |
| 📝 PDF 转文本 | 提取文档文字内容 |
| 📊 PDF 转图片 | 将PDF页面导出为图片 |

### ✏️ PDF 编辑

| 功能 | 说明 |
|:---|:---|
| 💧 添加水印 | 文字/图片水印，支持自定义样式 |
| 📦 压缩 PDF | 轻量/中等/高压缩三档可选 |
| 📝 页眉页脚 | 添加页眉页脚（开发中） |
| 📌 插入页面 | 在指定位置插入新页面 |

### 🔐 安全与权限

| 功能 | 说明 |
|:---|:---|
| 🔑 PDF 加密 | RC4 128位加密，设置打开密码 |
| 🔓 PDF 解密 | 输入密码解除加密 |
| 📋 元数据修改 | 修改标题、作者等信息 |
| 🧹 元数据清空 | 清除所有元数据 |

### 🛠️ 工具

| 功能 | 说明 |
|:---|:---|
| 👁️ PDF 预览 | 内置预览，无需外部阅读器 |
| 📊 查看信息 | 查看页数、大小等详细信息 |
| 📋 任务队列 | 批量处理任务管理 |
| 📜 操作日志 | 记录所有操作历史 |

---

## 📥 安装

### 下载预编译版本

前往 [Releases](https://github.com/wljslmzcn/kuai-pdf/releases) 下载最新版本：

| 系统 | 文件 |
|:---|:---|
| Windows | `KuaiPDF_x.x.x_x64-setup.exe` |
| Windows (MSI) | `KuaiPDF_x.x.x_x64.msi` |

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/wljslmzcn/kuai-pdf.git
cd kuai-pdf

# 安装依赖
npm install

# 开发模式
npm run tauri dev

# 构建生产版本
npm run tauri build
```

---

## 🛠️ 开发

### 环境要求

- **Node.js** 18+
- **Rust** 1.77+
- **Tauri CLI** 2.x

### 项目结构

```
kuai-pdf/
├── src/                          # 🎨 Vue 3 前端
│   ├── api/                      # API 调用封装
│   ├── components/
│   │   ├── pdf/                  # PDF 工具组件
│   │   ├── MainContent.vue       # 主内容区
│   │   └── Sidebar.vue           # 侧边栏菜单
│   ├── App.vue                   # 根组件
│   └── main.ts                   # 入口文件
│
├── src-tauri/                    # 🦀 Rust 后端
│   ├── src/
│   │   ├── lib.rs                # Tauri 命令注册
│   │   ├── pdf_utils.rs          # PDF 基础工具
│   │   ├── page_ops.rs           # 页面操作实现
│   │   ├── converter.rs          # 格式转换实现
│   │   ├── editor.rs             # PDF 编辑实现
│   │   └── security.rs           # 加密解密实现
│   ├── Cargo.toml                # Rust 依赖配置
│   └── tauri.conf.json           # Tauri 应用配置
│
├── package.json                  # Node.js 依赖
└── vite.config.ts                # Vite 构建配置
```

### 常用命令

```bash
# 开发
npm run tauri dev          # 启动开发服务器

# 构建
npm run tauri build        # 构建生产版本

# 前端单独构建
npm run build              # 仅构建前端

# 代码检查
npm run lint               # ESLint 检查
```

---

## 🤝 参与贡献

欢迎贡献代码、提交问题或建议！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

---

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐ Star 支持一下！**

Made with ❤️ by [wljslmzcn](https://github.com/wljslmzcn)

</div>
