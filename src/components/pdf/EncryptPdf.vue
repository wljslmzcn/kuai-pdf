<script setup lang="ts">
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { encryptPdf, copyFile, openFile, openFolder } from '../../api/pdf'
import PdfPreview from './PdfPreview.vue'

const inputPath = ref('')
const resultPath = ref('')
const userPassword = ref('')
const ownerPassword = ref('')
const permissions = ref<string[]>([])
const loading = ref(false)

const permissionOptions = [
  { label: '允许打印', value: 'print' },
  { label: '允许复制文本', value: 'copy' },
  { label: '允许修改文档', value: 'modify' },
]

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

const handleEncrypt = async () => {
  if (!inputPath.value) {
    ElMessage.warning('请选择PDF文件')
    return
  }
  if (!userPassword.value && !ownerPassword.value) {
    ElMessage.warning('请至少设置一个密码')
    return
  }

  loading.value = true
  resultPath.value = ''
  try {
    const ts = Date.now()
    const tempPath = inputPath.value.replace(/\.pdf$/i, `_encrypted_${ts}.pdf`)
    await encryptPdf(
      inputPath.value,
      tempPath,
      userPassword.value,
      ownerPassword.value,
      permissions.value
    )
    resultPath.value = tempPath
    ElMessage.success('加密成功')
  } catch (error) {
    ElMessage.error(`加密失败: ${error}`)
  } finally {
    loading.value = false
  }
}

const handleExport = async () => {
  if (!resultPath.value) {
    ElMessage.warning('请先点击"加密PDF"生成预览')
    return
  }
  const { save } = await import('@tauri-apps/plugin-dialog')
  const selected = await save({
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
    defaultPath: 'encrypted.pdf'
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
  <div class="encrypt-pdf">
    <div class="form-section">
      <h3>选择文件</h3>
      <div class="file-input">
        <el-input v-model="inputPath" placeholder="选择PDF文件" readonly />
        <el-button @click="handleFileSelect">选择文件</el-button>
      </div>
    </div>

    <div class="form-section">
      <h3>打开密码</h3>
      <el-input v-model="userPassword" type="password" placeholder="输入打开密码" show-password />
      <div class="hint">用户打开文件时需要输入此密码</div>
    </div>

    <div class="form-section">
      <h3>权限密码</h3>
      <el-input v-model="ownerPassword" type="password" placeholder="输入权限密码" show-password />
      <div class="hint">修改权限设置时需要输入此密码</div>
    </div>

    <div class="form-section">
      <h3>权限设置</h3>
      <el-checkbox-group v-model="permissions">
        <el-checkbox v-for="option in permissionOptions" :key="option.value" :value="option.value">
          {{ option.label }}
        </el-checkbox>
      </el-checkbox-group>
    </div>

    <div class="form-section button-group">
      <el-button type="primary" @click="handleEncrypt" :loading="loading" size="large">
        加密PDF
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
.encrypt-pdf {
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
