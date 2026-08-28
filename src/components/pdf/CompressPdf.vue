<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { compressPdf, copyFile, openFile, openFolder, type CompressResult } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const compressLevel = ref<'light' | 'medium' | 'high'>('medium')
const loading = ref(false)
const resultPath = ref('')
const result = ref<CompressResult | null>(null)

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
    result.value = null
  }
}

const handleCompress = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  result.value = null
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_compressed_${ts}.pdf`)
    result.value = await compressPdf(inputPath.value, tempPath, compressLevel.value)
    resultPath.value = tempPath
    ElMessage.success('压缩成功')
  } catch (error) {
    ElMessage.error(`压缩失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"开始压缩"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'compressed.pdf'
  })
  if (selected) {
    try {
      await copyFile(resultPath.value, selected)
      ElMessage.success('导出成功')
    } catch (error) {
      ElMessage.error(`导出失败: ${error}`)
    }
  }
}

const handleOpenFile = async () => {
  if (resultPath.value) {
    try { await openFile(resultPath.value) } catch (e) { ElMessage.error(`${e}`) }
  }
}

const handleOpenFolder = async () => {
  if (resultPath.value) {
    try { await openFolder(resultPath.value) } catch (e) { ElMessage.error(`${e}`) }
  }
}

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}
</script>

<template>
  <div class="compress-pdf">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>压缩级别</h3>
      <el-radio-group v-model="compressLevel">
        <el-radio value="light">轻量（几乎无损）</el-radio>
        <el-radio value="medium">中等</el-radio>
        <el-radio value="high">高压缩（降低图片质量）</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleCompress" :loading="loading" size="large">
        开始压缩
      </el-button>
      <el-button @click="handleExport" :disabled="!resultPath" size="large">
        导出
      </el-button>
      <el-button @click="handleOpenFile" :disabled="!resultPath" size="large">
        打开文件
      </el-button>
      <el-button @click="handleOpenFolder" :disabled="!resultPath" size="large">
        打开目录
      </el-button>
    </div>

    <div class="form-section" v-if="result">
      <h3>压缩结果</h3>
      <div class="result-card">
        <div class="result-item">
          <span class="label">原始大小</span>
          <span class="value">{{ formatSize(result.original_size) }}</span>
        </div>
        <div class="result-item">
          <span class="label">压缩后大小</span>
          <span class="value">{{ formatSize(result.compressed_size) }}</span>
        </div>
        <div class="result-item highlight">
          <span class="label">压缩率</span>
          <span class="value">{{ result.ratio }}%</span>
        </div>
      </div>
    </div>

    <PdfPreview v-if="resultPath" :path="resultPath" />
    <PdfPreview v-else-if="inputPath" :path="inputPath" />
  </div>
</template>

<style scoped>
.compress-pdf {
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

.result-card {
  background: var(--bg);
  border-radius: var(--radius-sm);
  padding: 20px;
  display: flex;
  gap: 24px;
}

.result-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.result-item .label {
  font-size: 13px;
  color: var(--text-secondary);
}

.result-item .value {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

.result-item.highlight .value {
  color: var(--success);
}

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
