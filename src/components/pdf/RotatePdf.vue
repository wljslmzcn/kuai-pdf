<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { rotatePdfPages, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const resultPath = ref('')
const pages = ref('1')
const angle = ref(90)
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
  }
}

const handleRotate = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_rotated_${ts}.pdf`)
    await rotatePdfPages(inputPath.value, tempPath, pages.value, angle.value)
    resultPath.value = tempPath
    ElMessage.success('旋转成功')
  } catch (error) {
    ElMessage.error(`旋转失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"开始旋转"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'rotated.pdf'
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
  <div class="rotate-pdf">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>旋转角度</h3>
      <el-radio-group v-model="angle">
        <el-radio :value="90">90°</el-radio>
        <el-radio :value="180">180°</el-radio>
        <el-radio :value="270">270°</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section">
      <h3>页码范围</h3>
      <el-input v-model="pages" placeholder="例如: 1-3,5,7-9" />
      <div class="hint">留空表示旋转所有页面</div>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleRotate" :loading="loading" size="large">
        开始旋转
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
.rotate-pdf {
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
</style>
