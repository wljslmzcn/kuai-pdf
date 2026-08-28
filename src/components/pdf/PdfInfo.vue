<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { getPdfInfo, type PdfInfo } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const pdfInfo = ref<PdfInfo | null>(null)
const loading = ref(false)

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
    loadInfo()
  }
}

const loadInfo = async () => {
  if (!inputPath.value) return

  loading.value = true
  try {
    pdfInfo.value = await getPdfInfo(inputPath.value)
  } catch (error) {
    ElMessage.error(`获取信息失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const formatSize = (bytes: number) => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}
</script>

<template>
  <div class="pdf-info">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section" v-if="pdfInfo">
      <h3>文件信息</h3>
      <div class="info-card">
        <div class="info-item">
          <span class="label">文件名</span>
          <span class="value">{{ pdfInfo.file_name }}</span>
        </div>
        <div class="info-item">
          <span class="label">页数</span>
          <span class="value">{{ pdfInfo.page_count }} 页</span>
        </div>
        <div class="info-item">
          <span class="label">文件大小</span>
          <span class="value">{{ formatSize(pdfInfo.file_size) }}</span>
        </div>
      </div>
    </div>

    <PdfPreview v-if="inputPath" :path="inputPath" />
  </div>
</template>

<style scoped>
.pdf-info {
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

.info-card {
  background: var(--bg);
  border-radius: var(--radius-sm);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-item .label {
  font-size: 13px;
  color: var(--text-secondary);
}

.info-item .value {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
}
</style>
