<template>
  <div class="home-container">
    <div class="hero-section">
      <h1 class="title">AdProof 区块链凭证系统</h1>
      <p class="subtitle">基于零知识证明的隐私保护凭证系统</p>
    </div>

    <div class="app-section">
      <h2 class="section-title">选择应用</h2>
      <el-row :gutter="30">
        <el-col :xs="24" :sm="12">
          <el-card shadow="hover" class="app-card">
            <div class="app-card-content">
              <el-icon class="app-icon"><School /></el-icon>
              <h3>学历凭证系统</h3>
              <p>管理和验证学历证书，确保教育证明的真实性</p>
              <div class="app-features">
                <div class="feature">
                  <el-icon><Document /></el-icon>
                  <span>颁发证书</span>
                </div>
                <div class="feature">
                  <el-icon><Check /></el-icon>
                  <span>验证证书</span>
                </div>
                <div class="feature">
                  <el-icon><Search /></el-icon>
                  <span>查询证书</span>
                </div>
              </div>
              <el-button type="primary" class="app-button" @click.stop="router.push('/credential')">进入系统</el-button>
            </div>
          </el-card>
        </el-col>
        <el-col :xs="24" :sm="12">
          <el-card shadow="hover" class="app-card">
            <div class="app-card-content">
              <el-icon class="app-icon"><Money /></el-icon>
              <h3>金融凭证系统</h3>
              <p>安全管理个人财务凭证，保护隐私的同时实现有效验证</p>
              <div class="app-features">
                <div class="feature">
                  <el-icon><Money /></el-icon>
                  <span>收入凭证</span>
                </div>
                <div class="feature">
                  <el-icon><Star /></el-icon>
                  <span>信用凭证</span>
                </div>
                <div class="feature">
                  <el-icon><Location /></el-icon>
                  <span>跨境凭证</span>
                </div>
              </div>
              <el-button type="primary" class="app-button" @click.stop="navigateToFinancial">进入系统</el-button>
            </div>
          </el-card>
        </el-col>
      </el-row>
    </div>

    <div class="info-section">
      <h2 class="section-title">系统特点</h2>
      <el-row :gutter="20">
        <el-col :xs="24" :sm="8">
          <div class="info-card">
            <el-icon class="info-icon"><Lock /></el-icon>
            <h3>隐私保护</h3>
            <p>基于零知识证明技术，无需公开敏感信息</p>
          </div>
        </el-col>
        <el-col :xs="24" :sm="8">
          <div class="info-card">
            <el-icon class="info-icon"><Connection /></el-icon>
            <h3>区块链支持</h3>
            <p>利用区块链不可篡改特性确保凭证真实性</p>
          </div>
        </el-col>
        <el-col :xs="24" :sm="8">
          <div class="info-card">
            <el-icon class="info-icon"><Key /></el-icon>
            <h3>安全可靠</h3>
            <p>采用先进加密技术保障数据安全与可靠性</p>
          </div>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { 
  Document, 
  Check, 
  Search, 
  School, 
  Money, 
  Wallet, 
  Star, 
  Location,
  Lock,
  Connection,
  Key
} from '@element-plus/icons-vue'

const router = useRouter()

// 导航到金融系统
const navigateToFinancial = () => {
  const currentRole = localStorage.getItem('role') || ''
  // 检查当前角色是否适合金融系统
  if (!['financial_institution', 'individual', 'verifier'].includes(currentRole)) {
    // 默认设置为金融机构角色
    localStorage.setItem('role', 'financial_institution')
  }
  console.log('Navigating to financial system')
  // 使用延时确保角色更新后再导航
  setTimeout(() => {
    router.push('/financial')
  }, 100)
}
</script>

<style scoped>
.home-container {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.hero-section {
  text-align: center;
  margin-bottom: 40px;
  padding: 40px 0;
}

.title {
  font-size: 2.5rem;
  color: var(--el-color-primary);
  margin-bottom: 10px;
}

.subtitle {
  font-size: 1.2rem;
  color: #666;
}

.section-title {
  text-align: center;
  margin-bottom: 30px;
  font-weight: 600;
  position: relative;
  padding-bottom: 15px;
}

.section-title:after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 60px;
  height: 3px;
  background-color: var(--el-color-primary);
}

.app-section {
  margin-bottom: 60px;
}

.app-card {
  height: 100%;
  transition: all 0.3s;
  cursor: pointer;
  overflow: hidden;
}

.app-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(0,0,0,0.1);
}

.app-card-content {
  padding: 20px;
  text-align: center;
}

.app-icon {
  font-size: 3rem;
  margin-bottom: 15px;
  color: var(--el-color-primary);
}

.app-features {
  display: flex;
  justify-content: space-around;
  margin: 20px 0;
}

.feature {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 5px;
}

.feature .el-icon {
  font-size: 1.5rem;
  color: var(--el-color-primary);
}

.app-button {
  margin-top: 15px;
  width: 120px;
}

.info-section {
  margin-bottom: 40px;
}

.info-card {
  padding: 30px 20px;
  text-align: center;
  border-radius: 8px;
  background-color: #f8f9fa;
  height: 100%;
  transition: all 0.3s;
}

.info-card:hover {
  background-color: #eef2f7;
}

.info-icon {
  font-size: 2.5rem;
  margin-bottom: 15px;
  color: var(--el-color-primary);
}

@media (max-width: 768px) {
  .app-section .el-col {
    margin-bottom: 20px;
  }
  
  .info-section .el-col {
    margin-bottom: 15px;
  }
}
</style>
