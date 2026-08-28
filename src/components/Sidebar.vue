<script setup lang="ts">
const props = defineProps<{
  activeMenu: string
}>()

const emit = defineEmits<{
  'update:active-menu': [value: string]
}>()

const menuGroups = [
  {
    title: '页面操作',
    items: [
      { id: 'page-ops', label: 'PDF 拆分' },
      { id: 'page-merge', label: 'PDF 合并' },
      { id: 'page-rotate', label: '页面旋转' },
      { id: 'page-delete', label: '页面删除' },
      { id: 'page-extract', label: '页面提取' },
      { id: 'page-reorder', label: '页面重排' },
      { id: 'page-crop', label: 'PDF 裁剪' },
    ]
  },
  {
    title: '格式转换',
    items: [
      { id: 'convert-to-img', label: 'PDF 转图片' },
      { id: 'convert-from-img', label: '图片转 PDF' },
      { id: 'convert-to-text', label: 'PDF 转文本' },
      { id: 'convert-to-word', label: 'PDF 转 Word' },
      { id: 'convert-to-excel', label: 'PDF 转 Excel' },
      { id: 'convert-to-html', label: 'PDF 转 HTML' },
      { id: 'convert-from-doc', label: '文档转 PDF' },
    ]
  },
  {
    title: 'PDF 编辑',
    items: [
      { id: 'edit-watermark', label: '添加水印' },
      { id: 'edit-header', label: '页眉页脚' },
      { id: 'edit-insert', label: '插入页面' },
      { id: 'edit-compress', label: '压缩 PDF' },
      { id: 'edit-remove-watermark', label: '去除水印' },
    ]
  },
  {
    title: '安全 & 权限',
    items: [
      { id: 'security-encrypt', label: 'PDF 加密' },
      { id: 'security-decrypt', label: 'PDF 解密' },
      { id: 'security-metadata', label: '元数据修改' },
      { id: 'security-redact', label: '涂黑脱敏' },
    ]
  },
  {
    title: '高级功能',
    items: [
      { id: 'advanced-compare', label: 'PDF 对比' },
      { id: 'advanced-sign', label: 'PDF 签名' },
      { id: 'advanced-bookmark', label: '书签管理' },
      { id: 'advanced-info', label: '查看信息' },
      { id: 'advanced-repair', label: '修复 PDF' },
    ]
  },
  {
    title: '工具',
    items: [
      { id: 'tools-preview', label: 'PDF 预览' },
      { id: 'tools-queue', label: '任务队列' },
      { id: 'tools-log', label: '操作日志' },
    ]
  }
]
</script>

<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <div class="logo">
        <span class="logo-icon">快</span>
        <span class="logo-text">快PDF</span>
      </div>
    </div>
    <div class="sidebar-menu">
      <div v-for="group in menuGroups" :key="group.title" class="menu-group">
        <div class="menu-group-title">{{ group.title }}</div>
        <div
          v-for="item in group.items"
          :key="item.id"
          class="menu-item"
          :class="{ active: activeMenu === item.id }"
          @click="emit('update:active-menu', item.id)"
        >
          <span class="menu-label">{{ item.label }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.sidebar {
  width: 240px;
  background: var(--card);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid var(--border);
}

.logo {
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  width: 40px;
  height: 40px;
  background: var(--primary);
  color: white;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  font-weight: bold;
}

.logo-text {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
}

.sidebar-menu {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
}

.menu-group {
  margin-bottom: 16px;
}

.menu-group-title {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  padding: 8px 12px;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
}

.menu-item:hover {
  background: var(--primary-bg);
}

.menu-item.active {
  background: var(--primary);
  color: white;
}

.menu-label {
  font-size: 14px;
}
</style>
