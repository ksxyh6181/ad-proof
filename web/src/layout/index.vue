<template>
  <el-container class="layout-shell">
    <el-header class="layout-header" height="72px">
      <Header />
    </el-header>
    <el-main class="layout-main">
      <div class="layout-content">
        <router-view />
      </div>
    </el-main>
  </el-container>
</template>

<script lang="ts" setup>
import Header from './components/Header.vue'
</script>

<style scoped>
.layout-shell {
  min-height: 100vh;
}

.layout-header {
  padding: 0;
  position: fixed;
  inset: 0 0 auto;
  z-index: 20;
  backdrop-filter: blur(18px);
}

.layout-main {
  margin-top: 72px;
  padding: 0;
}

.layout-content {
  width: min(1200px, calc(100% - 40px));
  margin: 0 auto;
  padding: 28px 0 56px;
}

@media (max-width: 768px) {
  .layout-content {
    width: min(100%, calc(100% - 24px));
    padding: 18px 0 36px;
  }
}
</style>
