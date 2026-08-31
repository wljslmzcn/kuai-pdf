<script setup lang="ts">
const emit = defineEmits<{
  'navigate': [menuId: string]
}>()

const features = [
  { icon: '📄', title: 'PDF 拆分', desc: '按页码范围拆分', menu: 'page-ops' },
  { icon: '📎', title: 'PDF 合并', desc: '多个PDF一键合并', menu: 'page-merge' },
  { icon: '🔄', title: '页面旋转', desc: '支持90°/180°/270°', menu: 'page-rotate' },
  { icon: '💧', title: '添加水印', desc: '文字/图片水印', menu: 'edit-watermark' },
  { icon: '🔐', title: 'PDF 加密', desc: '密码保护文档', menu: 'security-encrypt' },
  { icon: '📦', title: 'PDF 压缩', desc: '减小文件体积', menu: 'edit-compress' },
  { icon: '🖼️', title: '图片转PDF', desc: '支持PNG/JPG/BMP', menu: 'convert-from-img' },
  { icon: '📝', title: 'PDF转文本', desc: '提取文字内容', menu: 'convert-to-text' },
]

const quickActions = [
  { icon: '📋', label: 'PDF 拆分', menu: 'page-ops' },
  { icon: '📎', label: 'PDF 合并', menu: 'page-merge' },
  { icon: '💧', label: '添加水印', menu: 'edit-watermark' },
  { icon: '🔐', label: 'PDF 加密', menu: 'security-encrypt' },
  { icon: '📦', label: 'PDF 压缩', menu: 'edit-compress' },
  { icon: '👁️', label: 'PDF 预览', menu: 'tools-preview' },
]

const navigate = (menu: string) => {
  emit('navigate', menu)
}
</script>

<template>
  <div class="home">
    <!-- 顶部 Banner -->
    <div class="banner">
      <div class="banner-content">
        <h1 class="banner-title">🚀 快PDF</h1>
        <p class="banner-subtitle">本地运行 · 完全免费 · 隐私安全</p>
        <p class="banner-desc">一站式PDF处理工具，所有操作在本地完成，文件不上传服务器</p>
      </div>
    </div>

    <!-- 快捷操作 -->
    <div class="section">
      <h2 class="section-title">⚡ 快捷操作</h2>
      <div class="quick-grid">
        <div
          v-for="action in quickActions"
          :key="action.menu"
          class="quick-item"
          @click="navigate(action.menu)"
        >
          <span class="quick-icon">{{ action.icon }}</span>
          <span class="quick-label">{{ action.label }}</span>
        </div>
      </div>
    </div>

    <!-- 功能介绍 -->
    <div class="section">
      <h2 class="section-title">✨ 功能特性</h2>
      <div class="feature-grid">
        <div
          v-for="feature in features"
          :key="feature.menu"
          class="feature-card"
          @click="navigate(feature.menu)"
        >
          <span class="feature-icon">{{ feature.icon }}</span>
          <div class="feature-info">
            <h3 class="feature-title">{{ feature.title }}</h3>
            <p class="feature-desc">{{ feature.desc }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- 核心优势 -->
    <div class="section">
      <h2 class="section-title">🛡️ 核心优势</h2>
      <div class="advantage-grid">
        <div class="advantage-card">
          <span class="advantage-icon">🔒</span>
          <h3>隐私安全</h3>
          <p>所有处理在本地完成<br/>文件不上传服务器</p>
        </div>
        <div class="advantage-card">
          <span class="advantage-icon">💰</span>
          <h3>完全免费</h3>
          <p>所有功能免费使用<br/>无任何隐藏收费</p>
        </div>
        <div class="advantage-card">
          <span class="advantage-icon">⚡</span>
          <h3>轻量高效</h3>
          <p>安装包仅5MB<br/>Rust后端极速处理</p>
        </div>
        <div class="advantage-card">
          <span class="advantage-icon">🖥️</span>
          <h3>跨平台</h3>
          <p>支持 Windows<br/>macOS / Linux</p>
        </div>
      </div>
    </div>

    <!-- 公众号推广 -->
    <div class="section">
      <div class="promo-card">
        <div class="promo-content">
          <h3>📱 关注公众号获取更多工具</h3>
          <p>关注「<strong>网络技术联盟站</strong>」回复「加群」加入交流群</p>
          <p class="promo-tip">获取最新版本、功能更新、技术支持</p>
        </div>
        <div class="promo-qrcode">
          <img src="/wechat.jpg" alt="公众号二维码" />
        </div>
      </div>
    </div>

    <!-- 页脚 -->
    <div class="footer">
      <p>快PDF v1.0.0 · 开源免费 · MIT License</p>
      <p>GitHub: <a href="https://github.com/wljslmzcn/kuai-pdf" target="_blank">wljslmzcn/kuai-pdf</a></p>
    </div>
  </div>
</template>

<style scoped>
.home {
  padding: 24px;
  width: 100%;
  overflow-y: auto;
  height: 100%;
  flex: 1;
}

/* Banner */
.banner {
  background: linear-gradient(135deg, var(--primary) 0%, #8B5CF6 100%);
  border-radius: var(--radius);
  padding: 40px;
  margin-bottom: 32px;
  color: white;
}

.banner-title {
  font-size: 32px;
  font-weight: 700;
  margin-bottom: 8px;
}

.banner-subtitle {
  font-size: 16px;
  opacity: 0.9;
  margin-bottom: 8px;
}

.banner-desc {
  font-size: 14px;
  opacity: 0.8;
}

/* Section */
.section {
  margin-bottom: 32px;
}

.section-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 16px;
}

/* Quick Actions */
.quick-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 12px;
}

.quick-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 16px 8px;
  background: var(--card);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid var(--border);
}

.quick-item:hover {
  background: var(--primary-bg);
  border-color: var(--primary);
  transform: translateY(-2px);
}

.quick-icon {
  font-size: 24px;
}

.quick-label {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Features */
.feature-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.feature-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 16px;
  background: var(--card);
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid var(--border);
}

.feature-card:hover {
  background: var(--primary-bg);
  border-color: var(--primary);
}

.feature-icon {
  font-size: 28px;
}

.feature-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 2px;
}

.feature-desc {
  font-size: 12px;
  color: var(--text-secondary);
}

/* Advantages */
.advantage-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 16px;
}

.advantage-card {
  text-align: center;
  padding: 24px 16px;
  background: var(--card);
  border-radius: var(--radius);
  border: 1px solid var(--border);
}

.advantage-icon {
  font-size: 36px;
  display: block;
  margin-bottom: 12px;
}

.advantage-card h3 {
  font-size: 15px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 8px;
}

.advantage-card p {
  font-size: 12px;
  color: var(--text-secondary);
  line-height: 1.6;
}

/* Promo */
.promo-card {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24px 32px;
  background: linear-gradient(135deg, #f0fdf4 0%, #dcfce7 100%);
  border-radius: var(--radius);
  border: 1px solid #bbf7d0;
}

.promo-content h3 {
  font-size: 18px;
  font-weight: 600;
  color: #166534;
  margin-bottom: 8px;
}

.promo-content p {
  font-size: 14px;
  color: #15803d;
  margin-bottom: 4px;
}

.promo-tip {
  font-size: 12px;
  color: #22c55e;
  margin-top: 8px;
}

.promo-qrcode img {
  width: 100px;
  height: 100px;
  border-radius: 8px;
}

/* Footer */
.footer {
  text-align: center;
  padding: 24px;
  color: var(--text-tertiary);
  font-size: 12px;
}

.footer a {
  color: var(--primary);
  text-decoration: none;
}

.footer a:hover {
  text-decoration: underline;
}

/* Responsive */
@media (max-width: 768px) {
  .quick-grid {
    grid-template-columns: repeat(3, 1fr);
  }

  .feature-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .advantage-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
