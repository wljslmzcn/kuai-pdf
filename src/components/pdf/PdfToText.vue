<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { pdfToText, openFile } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const outputPath = ref('')
const extractedText = ref('')
const loading = ref(false)
const resultPath = ref('')

const handleFileSelect = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: false,
    filters: [{
      name: 'PDF',
      extensions: ['pdf']
    }]
  })
  if (selected) {
    inputPath.value = selected as string
    resultPath.value = ''
  }
}

const handleOutputPath = async () => {
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{
      name: '文本文件',
      extensions: ['txt']
    }],
    defaultPath: 'output.txt'
  })
  if (selected) {
    outputPath.value = selected
  }
}

const handleExtract = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  try {
    const text = await pdfToText(inputPath.value, outputPath.value || undefined)
    extractedText.value = text
    resultPath.value = outputPath.value || ''
    if (outputPath.value) {
      ElMessage.success('提取成功，已保存到文件')
    } else {
      ElMessage.success('提取成功')
    }
  } catch (error) {
    ElMessage.error(`提取失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleCopyText = () => {
  navigator.clipboard.writeText(extractedText.value)
  ElMessage.success('已复制到剪贴板')
}

const handleOpenFile = async () => {
  if (resultPath.value) {
    try { await openFile(resultPath.value) } catch (e) { ElMessage.error(`${e}`) }
  }
}
</script>

<template>
  <div class="pdf-to-text">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>输出路径（可选）</h3>
      <div class="file-input">
        <el-input v-model="outputPath" placeholder="选择输出路径" readonly />
        <el-button @click="handleOutputPath">选择路径</el-button>
      </div>
      <div class="hint">留空则只在下方显示提取结果</div>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleExtract" :loading="loading" size="large">
        开始提取
      </el-button>
      <el-button @click="handleOpenFile" :disabled="!resultPath" size="large">
        打开文件
      </el-button>
    </div>

    <div class="form-section" v-if="extractedText">
      <h3>提取结果</h3>
      <div class="text-preview">
        <pre>{{ extractedText }}</pre>
      </div>
      <el-button @click="handleCopyText" style="margin-top: 12px">
        复制文本
      </el-button>
    </div>

    <PdfPreview v-if="inputPath" :path="inputPath" />
  </div>
</template>

<style scoped>
.pdf-to-text {
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

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
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
</style>
