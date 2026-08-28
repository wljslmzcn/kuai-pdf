<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { compressPdf, encryptPdf } from '../../api/pdf'

interface FileItem {
  path: string
  name: string
  status: 'pending' | 'processing' | 'completed' | 'error'
  result?: string
}

const files = ref<FileItem[]>([])
const outputDir = ref('')
const processType = ref<'compress' | 'encrypt'>('compress')
const compressLevel = ref<'light' | 'medium' | 'high'>('medium')
const password = ref('')
const loading = ref(false)

const handleAddFiles = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    multiple: true,
    filters: [{
      name: 'PDF',
      extensions: ['pdf']
    }]
  })
  if (selected && Array.isArray(selected)) {
    for (const path of selected) {
      const name = path.split(/[/\\]/).pop() || path
      if (!files.value.find(f => f.path === path)) {
        files.value.push({ path, name, status: 'pending' })
      }
    }
  }
}

const handleRemoveFile = (index: number) => {
  files.value.splice(index, 1)
}

const handleOutputDir = async () => {
  const { open } = await import('@tauri-apps/plugin-dialog')
  const selected = await open({
    directory: true,
  })
  if (selected) {
    outputDir.value = selected as string
  }
}

const handleProcess = async () => {
  if (files.value.length === 0) {
    ElMessage.warning('请添加文件')
    return
  }
  if (!outputDir.value) {
    ElMessage.warning('请选择输出目录')
    return
  }

  loading.value = true
  let successCount = 0
  let errorCount = 0

  for (const file of files.value) {
    if (file.status === 'completed') continue

    file.status = 'processing'
    try {
      const outputPath = `${outputDir.value}\\${file.name}`

      if (processType.value === 'compress') {
        await compressPdf(file.path, outputPath, compressLevel.value)
      } else {
        await encryptPdf(file.path, outputPath, password.value, password.value, [])
      }

      file.status = 'completed'
      file.result = '处理成功'
      successCount++
    } catch (error) {
      file.status = 'error'
      file.result = `处理失败: ${error}`
      errorCount++
    }
  }

  loading.value = false
  ElMessage.success(`处理完成: 成功 ${successCount} 个, 失败 ${errorCount} 个`)
}
</script>

<template>
  <div class="batch-process">
    <div class="form-section">
      <h3>添加文件</h3>
      <el-button @click="handleAddFiles">添加PDF文件</el-button>
    </div>

    <div class="form-section" v-if="files.length > 0">
      <h3>文件列表</h3>
      <div class="file-list">
        <div v-for="(file, index) in files" :key="file.path" class="file-item">
          <span class="file-name">{{ file.name }}</span>
          <el-tag :type="file.status === 'completed' ? 'success' : file.status === 'error' ? 'danger' : file.status === 'processing' ? 'warning' : 'info'" size="small">
            {{ file.status === 'pending' ? '待处理' : file.status === 'processing' ? '处理中' : file.status === 'completed' ? '已完成' : '失败' }}
          </el-tag>
          <span class="file-result" v-if="file.result">{{ file.result }}</span>
          <el-button size="small" type="danger" @click="handleRemoveFile(index)">
            删除
          </el-button>
        </div>
      </div>
    </div>

    <div class="form-section">
      <h3>处理类型</h3>
      <el-radio-group v-model="processType">
        <el-radio value="compress">批量压缩</el-radio>
        <el-radio value="encrypt">批量加密</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section" v-if="processType === 'compress'">
      <h3>压缩级别</h3>
      <el-radio-group v-model="compressLevel">
        <el-radio value="light">轻量</el-radio>
        <el-radio value="medium">中等</el-radio>
        <el-radio value="high">高压缩</el-radio>
      </el-radio-group>
    </div>

    <div class="form-section" v-if="processType === 'encrypt'">
      <h3>密码</h3>
      <el-input v-model="password" type="password" placeholder="输入密码" show-password />
    </div>

    <div class="form-section">
      <h3>输出目录</h3>
      <div class="file-input">
        <el-input v-model="outputDir" placeholder="选择输出目录" readonly />
        <el-button @click="handleOutputDir">选择目录</el-button>
      </div>
    </div>

    <div class="form-section">
      <el-button type="primary" @click="handleProcess" :loading="loading" size="large">
        开始批量处理
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.batch-process {
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

.file-name {
  flex: 1;
  font-size: 14px;
}

.file-result {
  font-size: 12px;
  color: var(--text-secondary);
}
</style>
