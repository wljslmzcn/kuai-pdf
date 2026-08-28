<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { checkTesseract, ocrImage } from '../../api/pdf'

const inputPath = ref('')
const outputPath = ref('')
const language = ref('chi_sim')
const loading = ref(false)
const extractedText = ref('')
const tesseractInstalled = ref(false)

onMounted(async () => {
  tesseractInstalled.value = await checkTesseract()
})

const handleFileSelect = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: false,
    filters: [{
      name: '图片',
      extensions: ['png', 'jpg', 'jpeg', 'bmp', 'tiff']
    }]
  })
  if (selected) {
    inputPath.value = selected as string
  }
}

const handleOutputPath = async () => {
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{
      name: '文本文件',
      extensions: ['txt']
    }],
    defaultPath: 'ocr_result.txt'
  })
  if (selected) {
    outputPath.value = selected
  }
}

const handleOcr = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择图片文件')
    return
  }

  loading.value = true
  try {
    const text = await ocrImage(inputPath.value, outputPath.value || undefined, language.value)
    extractedText.value = text
    if (outputPath.value) {
      ElMessage.success('OCR识别成功，已保存到文件')
    } else {
      ElMessage.success('OCR识别成功')
    }
  } catch (error) {
    ElMessage.error(`OCR识别失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleCopyText = () => {
  navigator.clipboard.writeText(extractedText.value)
  ElMessage.success('已复制到剪贴板')
}
</script>

<template>
  <div class="ocr-pdf">
    <div class="form-section" v-if="!tesseractInstalled">
      <el-alert
        title="Tesseract OCR未安装"
        type="warning"
        show-icon
      >
        <template #default>
          <p>OCR功能需要安装Tesseract OCR引擎：</p>
          <ul>
            <li><strong>Windows:</strong> 下载安装 <a href="https://github.com/UB-Mannheim/tesseract/wiki" target="_blank">Tesseract for Windows</a></li>
            <li><strong>macOS:</strong> <code>brew install tesseract</code></li>
            <li><strong>Linux:</strong> <code>sudo apt install tesseract-ocr</code></li>
          </ul>
          <p>安装后需要安装中文语言包：<code>tesseract-ocr-chi-sim</code></p>
        </template>
      </el-alert>
    </div>

    <div class="form-section">
      <h3>选择图片</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择图片文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
      <div class="hint">支持 PNG、JPG、BMP、TIFF 格式</div>
    </div>

    <div class="form-section">
      <h3>识别语言</h3>
      <el-select v-model="language" placeholder="选择语言">
        <el-option label="简体中文" value="chi_sim" />
        <el-option label="英文" value="eng" />
        <el-option label="中英文混合" value="chi_sim+eng" />
      </el-select>
    </div>

    <div class="form-section">
      <h3>输出路径（可选）</h3>
      <div class="file-input">
        <el-input v-model="outputPath" placeholder="选择输出路径" readonly />
        <el-button @click="handleOutputPath">选择路径</el-button>
      </div>
    </div>

    <div class="form-section">
      <el-button
        type="primary"
        @click="handleOcr"
        :loading="loading"
        :disabled="!tesseractInstalled"
        size="large"
      >
        开始OCR识别
      </el-button>
    </div>

    <div class="form-section" v-if="extractedText">
      <h3>识别结果</h3>
      <div class="text-preview">
        <pre>{{ extractedText }}</pre>
      </div>
      <el-button @click="handleCopyText" style="margin-top: 12px">
        复制文本
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.ocr-pdf {
  padding: 20px;
}

.form-section {
  margin-bottom: 24px;
}

.form-section h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 12px;
}

.file-input {
  display: flex;
  gap: 12px;
}

.file-input .el-input {
  flex: 1;
}

.hint {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 8px;
}

.text-preview {
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 16px;
  max-height: 400px;
  overflow-y: auto;
}

.text-preview pre {
  margin: 0;
  font-family: 'Courier New', Courier, monospace;
  font-size: 13px;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.el-alert p {
  margin: 8px 0;
}

.el-alert ul {
  margin: 8px 0;
  padding-left: 20px;
}

.el-alert li {
  margin: 4px 0;
}

.el-alert code {
  background: rgba(0, 0, 0, 0.1);
  padding: 2px 6px;
  border-radius: 4px;
  font-family: monospace;
}
</style>
