<script setup lang="ts">
import { computed } from 'vue'
import SplitPdf from './pdf/SplitPdf.vue'
import MergePdf from './pdf/MergePdf.vue'
import RotatePdf from './pdf/RotatePdf.vue'
import DeletePdfPages from './pdf/DeletePdfPages.vue'
import ExtractPdfPages from './pdf/ExtractPdfPages.vue'
import ImagesToPdf from './pdf/ImagesToPdf.vue'
import PdfToText from './pdf/PdfToText.vue'
import AddWatermark from './pdf/AddWatermark.vue'
import CompressPdf from './pdf/CompressPdf.vue'
import EncryptPdf from './pdf/EncryptPdf.vue'
import DecryptPdf from './pdf/DecryptPdf.vue'
import EditMetadata from './pdf/EditMetadata.vue'
import PdfInfo from './pdf/PdfInfo.vue'
import TaskQueue from './pdf/TaskQueue.vue'
import OperationLog from './pdf/OperationLog.vue'
import PlaceholderTool from './pdf/PlaceholderTool.vue'

const props = defineProps<{
  activeMenu: string
}>()

const PH = (name: string) => ({ title: name, description: '', component: PlaceholderTool, props: { name } })

const toolMap: Record<string, { title: string; description: string; component: any; props?: any }> = {
  // 页面操作
  'page-ops': { title: 'PDF 拆分', description: '将PDF按页码范围或每N页拆分成多个文件', component: SplitPdf },
  'page-merge': { title: 'PDF 合并', description: '将多个PDF文件合并成一个文件', component: MergePdf },
  'page-rotate': { title: '页面旋转', description: '批量旋转PDF页面90/180/270度', component: RotatePdf },
  'page-delete': { title: '页面删除', description: '删除指定页码的页面', component: DeletePdfPages },
  'page-extract': { title: '页面提取', description: '提取部分页面导出新PDF', component: ExtractPdfPages },
  'page-reorder': PH('页面重排'),
  'page-crop': PH('PDF 裁剪'),
  // 格式转换
  'convert-to-img': PH('PDF 转图片'),
  'convert-from-img': { title: '图片转 PDF', description: '多张图片一键生成PDF', component: ImagesToPdf },
  'convert-to-text': { title: 'PDF 转文本', description: '提取PDF文本内容', component: PdfToText },
  'convert-to-word': PH('PDF 转 Word'),
  'convert-to-excel': PH('PDF 转 Excel'),
  'convert-to-html': PH('PDF 转 HTML'),
  'convert-from-doc': PH('文档转 PDF'),
  // PDF 编辑
  'edit-watermark': { title: '添加水印', description: '添加文字或图片水印', component: AddWatermark },
  'edit-header': PH('页眉页脚'),
  'edit-insert': PH('插入页面'),
  'edit-compress': { title: '压缩 PDF', description: '减小PDF文件大小', component: CompressPdf },
  'edit-remove-watermark': PH('去除水印'),
  // 安全权限
  'security-encrypt': { title: 'PDF 加密', description: '设置打开密码和权限密码', component: EncryptPdf },
  'security-decrypt': { title: 'PDF 解密', description: '移除PDF密码保护', component: DecryptPdf },
  'security-metadata': { title: '元数据修改', description: '修改标题、作者等元数据', component: EditMetadata },
  'security-redact': PH('涂黑脱敏'),
  // 高级功能
  'advanced-compare': PH('PDF 对比'),
  'advanced-sign': PH('PDF 签名'),
  'advanced-bookmark': PH('书签管理'),
  'advanced-info': { title: '查看信息', description: '查看PDF详细信息', component: PdfInfo },
  'advanced-repair': PH('修复 PDF'),
  // 工具
  'tools-preview': PH('PDF 预览'),
  'tools-queue': { title: '任务队列', description: '查看和管理处理任务', component: TaskQueue },
  'tools-log': { title: '操作日志', description: '查看历史操作记录', component: OperationLog },
}

const currentTool = computed(() => {
  const tool = toolMap[props.activeMenu]
  if (!tool) return { title: '未知工具', description: '', component: null }
  return tool
})
</script>

<template>
  <div class="main-content">
    <div class="content-header">
      <h1 class="tool-title">{{ currentTool.title }}</h1>
      <p class="tool-description">{{ currentTool.description }}</p>
    </div>
    <div class="content-body">
      <component :is="currentTool.component" v-if="currentTool.component" v-bind="currentTool.props || {}" />
      <div class="upload-area" v-else>
        <div class="upload-icon">📄</div>
        <div class="upload-text">拖拽PDF文件到这里，或点击选择文件</div>
        <div class="upload-hint">支持批量处理，文件仅在本地处理</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  padding: 24px 32px;
  background: var(--card);
  border-bottom: 1px solid var(--border);
}

.tool-title {
  font-size: 24px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 8px;
}

.tool-description {
  font-size: 14px;
  color: var(--text-secondary);
}

.content-body {
  flex: 1;
  padding: 32px;
  overflow-y: auto;
}

.upload-area {
  width: 100%;
  max-width: 600px;
  margin: 0 auto;
  padding: 60px 40px;
  border: 2px dashed var(--border);
  border-radius: var(--radius);
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
}

.upload-area:hover {
  border-color: var(--primary);
  background: var(--primary-bg);
}

.upload-icon {
  font-size: 64px;
  margin-bottom: 16px;
}

.upload-text {
  font-size: 16px;
  color: var(--text);
  margin-bottom: 8px;
}

.upload-hint {
  font-size: 13px;
  color: var(--text-secondary);
}
</style>
