<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { mergePdfs, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

interface FileItem {
  path: string
  name: string
}

const files = ref<FileItem[]>([])
const loading = ref(false)
const resultPath = ref('')

const handleAddFiles = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: true,
    filters: [{ name: 'PDF', extensions: ['pdf'] }]
  })
  if (selected && Array.isArray(selected)) {
    for (const path of selected) {
      const name = path.split(/[/\\]/).pop() || path
      if (!files.value.find(f => f.path === path)) {
        files.value.push({ path, name })
      }
    }
  }
}

const handleRemoveFile = (index: number) => {
  files.value.splice(index, 1)
}

const handleMoveUp = (index: number) => {
  if (index > 0) {
    const temp = files.value[index]
    files.value[index] = files.value[index - 1]
    files.value[index - 1] = temp
  }
}

const handleMoveDown = (index: number) => {
  if (index < files.value.length - 1) {
    const temp = files.value[index]
    files.value[index] = files.value[index + 1]
    files.value[index + 1] = temp
  }
}

const handleMerge = async () => {
  if (files.value.length < 2) {
    ElMessage.warning('请选择至少2个PDF文件')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const firstDir = files.value[0].path.replace(/[\\/][^\\/]+$/, '')
    const ts = Date.now()
    const tempPath = `${firstDir}/_merged_${ts}.pdf`
    await mergePdfs(files.value.map(f => f.path), tempPath)
    resultPath.value = tempPath
    ElMessage.success('合并成功')
  } catch (error) {
    ElMessage.error(`合并失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"开始合并"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'merged.pdf'
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
  <div class="merge-pdf">
    <div class="form-section">
      <h3>添加文件</h3>
      <el-button @click="handleAddFiles">添加PDF文件</el-button>
    </div>

    <div class="form-section" v-if="files.length > 0">
      <h3>文件列表（可拖拽调整顺序）</h3>
      <div class="file-list">
        <div v-for="(file, index) in files" :key="file.path" class="file-item">
          <span class="file-index">{{ index + 1 }}</span>
          <span class="file-name">{{ file.name }}</span>
          <div class="file-actions">
            <el-button size="small" @click="handleMoveUp(index)" :disabled="index === 0">↑</el-button>
            <el-button size="small" @click="handleMoveDown(index)" :disabled="index === files.length - 1">↓</el-button>
            <el-button size="small" type="danger" @click="handleRemoveFile(index)">删除</el-button>
          </div>
        </div>
      </div>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleMerge" :loading="loading" :disabled="files.length < 2" size="large">
        开始合并
      </el-button>
      <el-button @click="handleExport" :disabled="!resultPath" size="large">导出</el-button>
      <el-button @click="handleOpenFile" :disabled="!resultPath" size="large">打开文件</el-button>
      <el-button @click="handleOpenFolder" :disabled="!resultPath" size="large">打开目录</el-button>
    </div>

    <PdfPreview v-if="resultPath" :path="resultPath" />
  </div>
</template>

<style scoped>
.merge-pdf {
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

.file-list {
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 12px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  border-bottom: 1px solid var(--border);
}

.file-item:last-child {
  border-bottom: none;
}

.file-index {
  width: 24px;
  height: 24px;
  background: var(--primary-bg);
  color: var(--primary);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: 600;
}

.file-name {
  flex: 1;
  font-size: 14px;
}

.file-actions {
  display: flex;
  gap: 8px;
}

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
