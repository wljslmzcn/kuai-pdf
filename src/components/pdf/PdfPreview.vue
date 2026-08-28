<script setup lang="ts">
import { ref, watch } from 'vue'
import { readFileAsBase64 } from '../../api/pdf'

const props = defineProps<{
  path: string
}>()

const dataUrl = ref('')
const loading = ref(false)
const error = ref('')

watch(() => props.path, async (newPath) => {
  if (!newPath) {
    dataUrl.value = ''
    return
  }

  loading.value = true
  error.value = ''
  try {
    const base64 = await readFileAsBase64(newPath)
    dataUrl.value = `data:application/pdf;base64,${base64}`
  } catch (e) {
    error.value = `预览失败: ${e}`
  } finally {
    loading.value = false
  }
}, { immediate: true })
</script>

<template>
  <div class="pdf-preview">
    <div class="preview-header">
      <span class="preview-title">PDF 预览</span>
    </div>
    <div class="preview-body">
      <div v-if="loading" class="preview-loading">加载中...</div>
      <div v-else-if="error" class="preview-error">{{ error }}</div>
      <iframe
        v-else-if="dataUrl"
        :src="dataUrl"
        class="preview-iframe"
      />
    </div>
  </div>
</template>

<style scoped>
.pdf-preview {
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  overflow: hidden;
  margin-top: 16px;
}

.preview-header {
  padding: 10px 16px;
  background: var(--bg);
  border-bottom: 1px solid var(--border);
}

.preview-title {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.preview-body {
  height: 600px;
  overflow: auto;
  background: #555;
}

.preview-iframe {
  width: 100%;
  height: 100%;
  border: none;
}

.preview-loading,
.preview-error {
  font-size: 14px;
  color: var(--text-secondary);
  padding: 20px;
}

.preview-error {
  color: var(--danger);
}
</style>
