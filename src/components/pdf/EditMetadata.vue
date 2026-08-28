<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { getPdfMetadata, updatePdfMetadata, clearPdfMetadata, copyFile, openFile, openFolder, type PdfMetadata } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const resultPath = ref('')
const metadata = ref<PdfMetadata>({
  title: '',
  author: '',
  subject: '',
  keywords: '',
})
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
    resultPath.value = ''
    loadMetadata()
  }
}

const loadMetadata = async () => {
  if (!inputPath.value) return

  try {
    const data = await getPdfMetadata(inputPath.value)
    metadata.value = data
  } catch (error) {
    ElMessage.error(`加载元数据失败: ${error}`)
  }
}

const handleUpdate = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_metadata_${ts}.pdf`)
    await updatePdfMetadata(inputPath.value, tempPath, metadata.value)
    resultPath.value = tempPath
    ElMessage.success('更新元数据成功')
  } catch (error) {
    ElMessage.error(`更新失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleClear = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_metadata_${ts}.pdf`)
    await clearPdfMetadata(inputPath.value, tempPath)
    metadata.value = { title: '', author: '', subject: '', keywords: '' }
    resultPath.value = tempPath
    ElMessage.success('清空元数据成功')
  } catch (error) {
    ElMessage.error(`清空失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先操作生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'metadata_updated.pdf'
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
</script>

<template>
  <div class="edit-metadata">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>元数据</h3>
      <div class="metadata-form">
        <div class="form-item">
          <label>标题</label>
          <el-input v-model="metadata.title" placeholder="输入标题" />
        </div>
        <div class="form-item">
          <label>作者</label>
          <el-input v-model="metadata.author" placeholder="输入作者" />
        </div>
        <div class="form-item">
          <label>主题</label>
          <el-input v-model="metadata.subject" placeholder="输入主题" />
        </div>
        <div class="form-item">
          <label>关键字</label>
          <el-input v-model="metadata.keywords" placeholder="输入关键字" />
        </div>
      </div>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleUpdate" :loading="loading" size="large">
        更新元数据
      </el-button>
      <el-button type="danger" @click="handleClear" :loading="loading" size="large">
        清空元数据
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

    <PdfPreview v-if="resultPath" :path="resultPath" />
    <PdfPreview v-else-if="inputPath" :path="inputPath" />
  </div>
</template>

<style scoped>
.edit-metadata {
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

.metadata-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.form-item {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.form-item label {
  font-size: 13px;
  color: var(--text-secondary);
}

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
