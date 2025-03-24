<template>
  <div class="financial-home">
    <div class="page-header">
      <h1>金融凭证系统</h1>
      <p class="subtitle">安全管理个人财务凭证，保护隐私的同时实现有效验证</p>
    </div>
    
    <el-row :gutter="30" class="role-section">
      <el-col :xs="24" :sm="8">
        <el-card 
          shadow="hover" 
          class="role-card" 
          :class="{ 'active': currentRole === 'financial_institution' }"
        >
          <el-icon class="role-icon"><OfficeBuilding /></el-icon>
          <h3>金融机构</h3>
          <p>颁发各类金融凭证</p>
          <div class="actions">
            <div class="action-item">颁发收入证明</div>
            <div class="action-item">颁发信用评分</div>
            <div class="action-item">颁发跨境信用</div>
          </div>
          <el-button 
            type="primary" 
            class="role-button"
            @click="switchToRole('financial_institution', '/financial/institution/issue-income')"
          >
            进入金融机构
          </el-button>
        </el-card>
      </el-col>
      
      <el-col :xs="24" :sm="8">
        <el-card 
          shadow="hover" 
          class="role-card" 
          :class="{ 'active': currentRole === 'individual' }"
        >
          <el-icon class="role-icon"><User /></el-icon>
          <h3>个人用户</h3>
          <p>管理您的金融凭证</p>
          <div class="actions">
            <div class="action-item">查看我的凭证</div>
            <div class="action-item">凭证使用记录</div>
          </div>
          <el-button 
            type="success" 
            class="role-button"
            @click="switchToRole('individual', '/financial/individual/list')"
          >
            进入个人中心
          </el-button>
        </el-card>
      </el-col>
      
      <el-col :xs="24" :sm="8">
        <el-card 
          shadow="hover" 
          class="role-card" 
          :class="{ 'active': currentRole === 'verifier' }"
        >
          <el-icon class="role-icon"><Check /></el-icon>
          <h3>验证方</h3>
          <p>验证金融凭证真实性</p>
          <div class="actions">
            <div class="action-item">验证凭证</div>
          </div>
          <el-button 
            type="warning" 
            class="role-button"
            @click="switchToRole('verifier', '/financial/verifier/verify')"
          >
            进入验证中心
          </el-button>
        </el-card>
      </el-col>
    </el-row>
    
    <div class="feature-section">
      <h2>金融凭证功能</h2>
      <el-row :gutter="20">
        <el-col :xs="24" :sm="8">
          <div class="feature-item">
            <el-icon><Wallet /></el-icon>
            <h3>收入证明</h3>
            <p>生成符合隐私保护要求的收入证明，无需披露具体收入数额</p>
          </div>
        </el-col>
        <el-col :xs="24" :sm="8">
          <div class="feature-item">
            <el-icon><Star /></el-icon>
            <h3>信用评分</h3>
            <p>安全共享信用评分信息，不泄露个人详细信息</p>
          </div>
        </el-col>
        <el-col :xs="24" :sm="8">
          <div class="feature-item">
            <el-icon><Location /></el-icon>
            <h3>跨境信用</h3>
            <p>支持跨国金融信用传递，简化国际商务流程</p>
          </div>
        </el-col>
      </el-row>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { 
  OfficeBuilding, 
  User, 
  Check, 
  Wallet, 
  Star, 
  Location 
} from '@element-plus/icons-vue'

const router = useRouter()
const currentRole = ref('')

onMounted(() => {
  const savedRole = localStorage.getItem('role')
  if (savedRole) {
    currentRole.value = savedRole
  }
})

const switchToRole = (role: string, path: string) => {
  localStorage.setItem('role', role)
  currentRole.value = role
  
  console.log('Switching to role:', role)
  console.log('Navigating to:', path)
  
  // 添加一个小延时，确保角色状态更新后再导航
  setTimeout(() => {
    router.push(path)
  }, 100)
  
  ElMessage.success(`已切换到${role === 'financial_institution' ? '金融机构' : role === 'individual' ? '个人用户' : '验证方'}角色`)
}

</script>

<style scoped>
.financial-home {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.page-header {
  text-align: center;
  margin-bottom: 40px;
}

.page-header h1 {
  font-size: 2rem;
  color: var(--el-color-primary);
  margin-bottom: 10px;
}

.subtitle {
  font-size: 1.1rem;
  color: #666;
}

.role-section {
  margin-bottom: 50px;
}

.role-card {
  height: 100%;
  padding: 20px;
  text-align: center;
  transition: all 0.3s;
  border-radius: 8px;
  display: flex;
  flex-direction: column;
  align-items: center;
}

.role-card.active {
  border: 2px solid var(--el-color-primary);
  transform: translateY(-5px);
}

.role-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 10px 20px rgba(0,0,0,0.1);
}

.role-icon {
  font-size: 3rem;
  margin-bottom: 15px;
  color: var(--el-color-primary);
}

.role-card h3 {
  margin-top: 0;
  margin-bottom: 10px;
  font-size: 1.5rem;
}

.role-card p {
  color: #666;
  margin-bottom: 20px;
}

.actions {
  width: 100%;
  text-align: left;
  margin-top: auto;
}

.action-item {
  padding: 8px 0;
  border-top: 1px solid #eee;
  color: var(--el-color-primary);
  font-size: 0.9rem;
}

.action-item:last-child {
  border-bottom: 1px solid #eee;
}

.role-button {
  margin-top: 20px;
  width: 130px;
}

.feature-section {
  margin-top: 40px;
}

.feature-section h2 {
  text-align: center;
  margin-bottom: 30px;
  position: relative;
  padding-bottom: 15px;
}

.feature-section h2:after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 60px;
  height: 3px;
  background-color: var(--el-color-primary);
}

.feature-item {
  background-color: #f8f9fa;
  padding: 30px 20px;
  border-radius: 8px;
  text-align: center;
  height: 100%;
  transition: all 0.3s;
}

.feature-item:hover {
  background-color: #eef2f7;
}

.feature-item .el-icon {
  font-size: 2.5rem;
  margin-bottom: 15px;
  color: var(--el-color-primary);
}

.feature-item h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

.feature-item p {
  color: #666;
  line-height: 1.6;
}

@media (max-width: 768px) {
  .role-section .el-col {
    margin-bottom: 20px;
  }
  
  .feature-section .el-col {
    margin-bottom: 15px;
  }
}
</style>
