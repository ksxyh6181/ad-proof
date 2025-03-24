<template>
  <el-container class="layout">
    <el-header class="layout-header" height="50px">
      <Header />
    </el-header>
    <el-main class="layout-main">
      <router-view />
    </el-main>
  </el-container>
</template>

<script lang="ts" setup>
import Header from './components/Header.vue'
</script>

<style scoped>
.layout {
  min-height: 100vh;
}
.layout-header {
  padding: 0;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}
.layout-main {
  margin-top: 50px;
  background-color: var(--el-bg-color-page);
}
</style>
