<script setup lang="ts">
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'
import MainContent from './components/MainContent.vue'

const activeMenu = ref('page-ops')
</script>

<template>
  <div class="app-container">
    <Sidebar :active-menu="activeMenu" @update:active-menu="activeMenu = $event" />
    <MainContent :active-menu="activeMenu" />
  </div>
</template>

<style>
:root {
  --primary: #5E6AD2;
  --primary-light: #7B84E0;
  --primary-bg: rgba(94, 106, 210, 0.08);
  --success: #4CAF82;
  --warning: #E5A15C;
  --danger: #E25D5D;
  --bg: #F2F2F7;
  --card: #FFFFFF;
  --text: #1D1D1F;
  --text-secondary: #86868B;
  --text-tertiary: #AEAEB2;
  --border: rgba(0, 0, 0, 0.06);
  --shadow: 0 1px 3px rgba(0, 0, 0, 0.06);
  --shadow-hover: 0 4px 12px rgba(0, 0, 0, 0.08);
  --radius: 12px;
  --radius-sm: 8px;
}

.dark {
  --bg: #1C1C1E;
  --card: #2C2C2E;
  --text: #F5F5F7;
  --text-secondary: #98989D;
  --text-tertiary: #636366;
  --border: rgba(255, 255, 255, 0.08);
  --shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  background: var(--bg);
  color: var(--text);
}

.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}
</style>
