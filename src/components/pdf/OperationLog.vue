<script setup lang="ts">
import { ref } from 'vue'

interface LogEntry {
  id: string
  action: string
  file: string
  status: 'success' | 'error'
  timestamp: Date
  details?: string
}

const logs = ref<LogEntry[]>([])

const addLog = (action: string, file: string, status: 'success' | 'error', details?: string) => {
  const log: LogEntry = {
    id: Date.now().toString(),
    action,
    file,
    status,
    timestamp: new Date(),
    details,
  }
  logs.value.unshift(log)
}

const clearLogs = () => {
  logs.value = []
}

const getStatusType = (status: string) => {
  return status === 'success' ? 'success' : 'danger'
}

const getStatusText = (status: string) => {
  return status === 'success' ? '成功' : '失败'
}

defineExpose({
  addLog
})
</script>

<template>
  <div class="operation-log">
    <div class="header">
      <h3>操作日志</h3>
      <el-button size="small" @click="clearLogs" :disabled="logs.length === 0">
        清空日志
      </el-button>
    </div>

    <div class="log-list" v-if="logs.length > 0">
      <div v-for="log in logs" :key="log.id" class="log-item">
        <div class="log-header">
          <span class="log-action">{{ log.action }}</span>
          <el-tag :type="getStatusType(log.status)" size="small">
            {{ getStatusText(log.status) }}
          </el-tag>
        </div>
        <div class="log-file">{{ log.file }}</div>
        <div class="log-time">{{ log.timestamp.toLocaleString() }}</div>
        <div class="log-details" v-if="log.details">{{ log.details }}</div>
      </div>
    </div>

    <div class="empty-state" v-else>
      <p>暂无操作记录</p>
    </div>
  </div>
</template>

<style scoped>
.operation-log {
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header h3 {
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
}

.log-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.log-item {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 16px;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.log-action {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
}

.log-file {
  font-size: 13px;
  color: var(--text-secondary);
  margin-bottom: 4px;
}

.log-time {
  font-size: 12px;
  color: var(--text-tertiary);
}

.log-details {
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--border);
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
}
</style>
