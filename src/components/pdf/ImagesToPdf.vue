<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { imagesToPdf, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

interface ImageItem {
  path: string
  name: string
}

const images = ref<ImageItem[]>([])
const loading = ref(false)
const resultPath = ref('')

const handleAddImages = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: true,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp'] }]
  })
  if (selected && Array.isArray(selected)) {
    for (const path of selected) {
      const name = path.split(/[/\\]/).pop() || path
      if (!images.value.find(img => img.path === path)) {
        images.value.push({ path, name })
      }
    }
  }
}

const handleRemoveImage = (index: number) => {
  images.value.splice(index, 1)
}

const handleMoveUp = (index: number) => {
  if (index > 0) {
    const temp = images.value[index]
    images.value[index] = images.value[index - 1]
    images.value[index - 1] = temp
  }
}

const handleMoveDown = (index: number) => {
  if (index < images.value.length - 1) {
    const temp = images.value[index]
    images.value[index] = images.value[index + 1]
    images.value[index + 1] = temp
  }
}

const handleConvert = async () => {
  if (images.value.length === 0) {
    ElMessage.warning('请添加图片')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const dir = images.value[0].path.replace(/[\\/][^\\/]+$/, '')
    const ts = Date.now()
    const tempPath = `${dir}/_images_${ts}.pdf`
    await imagesToPdf(images.value.map(img => img.path), tempPath)
    resultPath.value = tempPath
    ElMessage.success('转换成功')
  } catch (error) {
    ElMessage.error(`转换失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"开始转换"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'images.pdf'
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
  <div class="images-to-pdf">
    <div class="form-section">
      <h3>添加图片</h3>
      <el-button @click="handleAddImages">添加图片</el-button>
    </div>

    <div class="form-section" v-if="images.length > 0">
      <h3>图片列表（可拖拽调整顺序）</h3>
      <div class="image-list">
        <div v-for="(image, index) in images" :key="image.path" class="image-item">
          <span class="image-index">{{ index + 1 }}</span>
          <span class="image-name">{{ image.name }}</span>
          <div class="image-actions">
            <el-button size="small" @click="handleMoveUp(index)" :disabled="index === 0">↑</el-button>
            <el-button size="small" @click="handleMoveDown(index)" :disabled="index === images.length - 1">↓</el-button>
            <el-button size="small" type="danger" @click="handleRemoveImage(index)">删除</el-button>
          </div>
        </div>
      </div>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleConvert" :loading="loading" :disabled="images.length === 0" size="large">
        开始转换
      </el-button>
      <el-button @click="handleExport" :disabled="!resultPath" size="large">导出</el-button>
      <el-button @click="handleOpenFile" :disabled="!resultPath" size="large">打开文件</el-button>
      <el-button @click="handleOpenFolder" :disabled="!resultPath" size="large">打开目录</el-button>
    </div>

    <PdfPreview v-if="resultPath" :path="resultPath" />
  </div>
</template>

<style scoped>
.images-to-pdf {
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

.image-list {
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 12px;
}

.image-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px;
  border-bottom: 1px solid var(--border);
}

.image-item:last-child {
  border-bottom: none;
}

.image-index {
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

.image-name {
  flex: 1;
  font-size: 14px;
}

.image-actions {
  display: flex;
  gap: 8px;
}

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
