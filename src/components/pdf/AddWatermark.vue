<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { addWatermark, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const watermarkText = ref('机密文件')
const opacity = ref(0.3)
const rotation = ref(-45)
const fontSize = ref(48)
const loading = ref(false)
const resultPath = ref('')

// 新增设置
const watermarkType = ref<'text' | 'image'>('text')
const imagePath = ref('')
const textColor = ref('#000000')
const density = ref<'single' | 'row' | 'grid'>('single')
const position = ref<'center' | 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right'>('center')

const handleFileSelect = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: false,
    filters: [{ name: 'PDF', extensions: ['pdf'] }]
  })
  if (selected) {
    inputPath.value = selected as string
    resultPath.value = ''
  }
}

const handleImageSelect = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: false,
    filters: [{ name: '图片', extensions: ['png', 'jpg', 'jpeg', 'bmp'] }]
  })
  if (selected) {
    imagePath.value = selected as string
  }
}

// hex颜色转 "r,g,b"
const hexToRgb = (hex: string): string => {
  const h = hex.replace('#', '')
  const r = parseInt(h.substring(0, 2), 16)
  const g = parseInt(h.substring(2, 4), 16)
  const b = parseInt(h.substring(4, 6), 16)
  return `${r},${g},${b}`
}

const handlePreview = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }
  if (watermarkType.value === 'text' && !watermarkText.value) {
    ElMessage.warning('请输入水印文本')
    return
  }
  if (watermarkType.value === 'image' && !imagePath.value) {
    ElMessage.warning('请选择水印图片')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_watermarked_${ts}.pdf`)
    const actualPath = await addWatermark(
      inputPath.value,
      tempPath,
      watermarkText.value,
      opacity.value,
      rotation.value,
      fontSize.value,
      watermarkType.value,
      watermarkType.value === 'image' ? imagePath.value : undefined,
      hexToRgb(textColor.value),
      density.value,
      position.value
    )
    resultPath.value = actualPath || tempPath
    ElMessage.success('预览生成成功')
  } catch (error) {
    ElMessage.error(`处理失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"添加水印"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'watermarked.pdf'
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
  <div class="add-watermark">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>水印类型</h3>
      <el-radio-group v-model="watermarkType">
        <el-radio value="text">文字水印</el-radio>
        <el-radio value="image">图片水印</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section" v-if="watermarkType === 'text'">
      <h3>水印文本</h3>
      <el-input v-model="watermarkText" placeholder="请输入水印文本" />
    </div>

    <div class="form-section" v-if="watermarkType === 'image'">
      <h3>水印图片</h3>
      <div class="file-input">
        <el-input v-model="imagePath" placeholder="选择水印图片" readonly />
        <el-button @click="handleImageSelect">选择图片</el-button>
      </div>
    </div>

    <div class="form-section" v-if="watermarkType === 'text'">
      <h3>文字颜色</h3>
      <el-color-picker v-model="textColor" />
    </div>

    <div class="form-section">
      <h3>透明度 ({{ opacity }})</h3>
      <el-slider v-model="opacity" :min="0.05" :max="1" :step="0.05" />
    </div>

    <div class="form-section">
      <h3>旋转角度 ({{ rotation }}°)</h3>
      <el-slider v-model="rotation" :min="-90" :max="90" :step="5" />
    </div>

    <div class="form-section">
      <h3>大小 ({{ fontSize }}px)</h3>
      <el-slider v-model="fontSize" :min="12" :max="200" :step="4" />
    </div>

    <div class="form-section">
      <h3>密度</h3>
      <el-radio-group v-model="density">
        <el-radio value="single">单个</el-radio>
        <el-radio value="row">一行</el-radio>
        <el-radio value="grid">满屏</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section" v-if="density === 'single'">
      <h3>位置</h3>
      <el-radio-group v-model="position">
        <el-radio value="center">居中</el-radio>
        <el-radio value="top-left">左上</el-radio>
        <el-radio value="top-right">右上</el-radio>
        <el-radio value="bottom-left">左下</el-radio>
        <el-radio value="bottom-right">右下</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handlePreview" :loading="loading" size="large">
        添加水印
      </el-button>
      <el-button @click="handleExport" :disabled="!resultPath" size="large">导出</el-button>
      <el-button @click="handleOpenFile" :disabled="!resultPath" size="large">打开文件</el-button>
      <el-button @click="handleOpenFolder" :disabled="!resultPath" size="large">打开目录</el-button>
    </div>

    <PdfPreview v-if="resultPath" :path="resultPath" />
    <PdfPreview v-else-if="inputPath" :path="inputPath" />
  </div>
</template>

<style scoped>
.add-watermark {
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

.button-group {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
</style>
