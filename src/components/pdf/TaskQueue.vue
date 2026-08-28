<script setup lang="ts">
import { ref } from 'vue'

interface Task {
  id: string
  name: string
  status: 'pending' | 'processing' | 'completed' | 'error'
  progress: number
  createdAt: Date
}

const tasks = ref<Task[]>([])

const addTask = (name: string) => {
  const task: Task = {
    id: Date.now().toString(),
    name,
    status: 'pending',
    progress: 0,
    createdAt: new Date(),
  }
  tasks.value.push(task)
  return task.id
}

const updateTask = (id: string, updates: Partial<Task>) => {
  const index = tasks.value.findIndex(t => t.id === id)
  if (index !== -1) {
    tasks.value[index] = { ...tasks.value[index], ...updates }
  }
}

const removeTask = (id: string) => {
  const index = tasks.value.findIndex(t => t.id === id)
  if (index !== -1) {
    tasks.value.splice(index, 1)
  }
}

const clearCompleted = () => {
  tasks.value = tasks.value.filter(t => t.status !== 'completed')
}

const getStatusType = (status: string) => {
  switch (status) {
    case 'completed': return 'success'
    case 'error': return 'danger'
    case 'processing': return 'warning'
    default: return 'info'
  }
}

const getStatusText = (status: string) => {
  switch (status) {
    case 'completed': return '已完成'
    case 'error': return '失败'
    case 'processing': return '处理中'
    default: return '待处理'
  }
}

defineExpose({
  addTask,
  updateTask
})
</script>

<template>
  <div class="task-queue">
    <div class="header">
      <h3>任务队列</h3>
      <el-button size="small" @click="clearCompleted" :disabled="!tasks.some(t => t.status === 'completed')">
        清除已完成
      </el-button>
    </div>

    <div class="task-list" v-if="tasks.length > 0">
      <div v-for="task in tasks" :key="task.id" class="task-item">
        <div class="task-info">
          <span class="task-name">{{ task.name }}</span>
          <el-tag :type="getStatusType(task.status)" size="small">
            {{ getStatusText(task.status) }}
          </el-tag>
        </div>
        <el-progress
          v-if="task.status === 'processing'"
          :percentage="task.progress"
        />
        <div class="task-meta">
          <span class="task-time">{{ task.createdAt.toLocaleTimeString() }}</span>
          <el-button size="small" type="danger" text @click="removeTask(task.id)">
            删除
          </el-button>
        </div>
      </div>
    </div>

    <div class="empty-state" v-else>
      <p>暂无任务</p>
    </div>
  </div>
</template>

<style scoped>
.task-queue {
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

.task-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.task-item {
  background: var(--card);
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  padding: 16px;
}

.task-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.task-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text);
}

.task-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 12px;
}

.task-time {
  font-size: 12px;
  color: var(--text-secondary);
}

.empty-state {
  text-align: center;
  padding: 40px;
  color: var(--text-secondary);
}
</style>
