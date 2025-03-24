<template>
  <div class="credential-blockchain">
    <el-card class="page-header">
      <h1>区块链凭证管理</h1>
      <p>管理凭证在Solana区块链上的存储和验证</p>
    </el-card>
    
    <SolanaCredentialManager class="manager-component" />
    
    <!-- 调试控制台 -->
    <DebugConsole />
  </div>
</template>

<script setup lang="ts">
import SolanaCredentialManager from '@/components/SolanaCredentialManager.vue';
import DebugConsole from '@/components/DebugConsole.vue';
</script>

<style scoped>
.credential-blockchain {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.page-header h1 {
  font-size: 24px;
  color: #303133;
  margin-bottom: 10px;
}

.page-header p {
  color: #606266;
  margin: 0;
}

.manager-component {
  margin-top: 30px;
}
</style>
