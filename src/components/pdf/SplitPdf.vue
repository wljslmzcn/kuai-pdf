<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { splitPdf, splitPdfByInterval, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const mode = ref<'range' | 'interval'>('range')
const inputPath = ref('')
const rangeStr = ref('')
const interval = ref(1)
const loading = ref(false)
const resultPath = ref('')
const resultFiles = ref<string[]>([])

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
    resultFiles.value = []
  }
}

const getTempDir = () => {
  const dir = inputPath.value.replace(/[\\/][^\\/]+$/, '')
  return dir + '/_temp_split'
}

const handleSplit = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }

  loading.value = true
  resultFiles.value = []
  resultPath.value = ''
  try {
    const tempDir = getTempDir()
    let result: string[]
    if (mode.value === 'range') {
      if (!rangeStr.value) {
        ElMessage.warning('请输入页码范围')
        loading.value = false
        return
      }
      result = await splitPdf(inputPath.value, tempDir, rangeStr.value)
    } else {
      result = await splitPdfByInterval(inputPath.value, tempDir, interval.value)
    }
    resultFiles.value = result
    resultPath.value = result.length > 0 ? result[0] : ''
    ElMessage.success(`成功拆分出 ${result.length} 个文件`)
  } catch (error) {
    ElMessage.error(`拆分失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (resultFiles.value.length === 0) {
    ElMessage.warning('请先点击"开始拆分"生成预览')
    return
  }
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    directory: true,
  })
  if (selected) {
    try {
      for (const file of resultFiles.value) {
        const fileName = file.replace(/^.*[\\/]/, '')
        await copyFile(file, (selected as string) + '/' + fileName)
      }
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
  <div class="split-pdf">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>拆分模式</h3>
      <el-radio-group v-model="mode">
        <el-radio value="range">按页码范围</el-radio>
        <el-radio value="interval">每N页拆分</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section" v-if="mode === 'range'">
      <h3>页码范围</h3>
      <el-input v-model="rangeStr" placeholder="例如: 1-3,5,7-9" />
      <div class="hint">多个范围用逗号分隔，如 1-3,5,7-9</div>
    </div>

    <div class="form-section" v-else>
      <h3>每页数</h3>
      <el-input-number v-model="interval" :min="1" :max="100" />
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleSplit" :loading="loading" size="large">
        开始拆分
      </el-button>
      <el-button @click="handleExport" :disabled="resultFiles.length === 0" size="large">
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
.split-pdf {
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
