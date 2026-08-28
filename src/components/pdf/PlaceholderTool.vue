<script setup lang="ts">
defineProps<{
  name: string
}>()
</script>

<template>
  <div class="placeholder-tool">
    <div class="placeholder-icon">🚧</div>
    <div class="placeholder-title">{{ name }}</div>
    <div class="placeholder-desc">功能开发中，敬请期待</div>
  </div>
</template>

<style scoped>
.placeholder-tool {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 80px 20px;
  text-align: center;
}

.placeholder-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.placeholder-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 8px;
}

.placeholder-desc {
  font-size: 14px;
  color: var(--text-secondary);
}
</style>
