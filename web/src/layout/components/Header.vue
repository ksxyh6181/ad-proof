<template>
  <div class="header">
    <div class="logo-container">
      <h1 class="logo">AdProof</h1>
    </div>
    
    <div class="nav-container">
      <div class="app-nav">
        <el-menu
          :default-active="activeApp"
          mode="horizontal"
          @select="handleAppSelect"
        >
          <el-menu-item index="/home">首页</el-menu-item>
          <el-menu-item index="/credential">学历凭证</el-menu-item>
          <el-menu-item index="/financial">金融凭证</el-menu-item>
          <el-menu-item index="/identity">AI数字身份</el-menu-item>
        </el-menu>
      </div>
    </div>
    
    <div class="role-selector">
      <span class="label">当前角色：</span>
      <el-select v-model="currentRole" @change="handleRoleChange">
        <el-option
          v-for="role in roles"
          :key="role.value"
          :label="role.label"
          :value="role.value"
        />
      </el-select>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'

const route = useRoute()
const router = useRouter()

const roles = [
  { label: '教育机构', value: 'education_institution' },
  { label: '毕业生', value: 'student' },
  { label: '雇主', value: 'employer' },
  { label: '金融机构', value: 'financial_institution' },
  { label: '个人用户', value: 'individual' },
  { label: '验证方', value: 'verifier' }
]

const currentRole = ref('')
const activeApp = computed(() => {
  const path = route.path
  if (path.startsWith('/credential')) return '/credential'
  if (path.startsWith('/financial')) return '/financial'
  if (path.startsWith('/identity')) return '/identity'
  return '/home'
})

onMounted(() => {
  const savedRole = localStorage.getItem('role')
  if (savedRole && roles.some(role => role.value === savedRole)) {
    currentRole.value = savedRole
  } else {
    // 默认角色为教育机构
    currentRole.value = 'education_institution'
    localStorage.setItem('role', 'education_institution')
  }
})

const handleRoleChange = (value: string) => {
  localStorage.setItem('role', value)
  ElMessage.success(`已切换到${roles.find(role => role.value === value)?.label}角色`)
  
  // 根据角色自动切换到相应的应用
  if (['education_institution', 'student', 'employer'].includes(value)) {
    if (!route.path.startsWith('/credential') && route.path !== '/home') {
      router.push('/credential')
    }
  } else if (['financial_institution', 'individual', 'verifier'].includes(value)) {
    if (!route.path.startsWith('/financial') && route.path !== '/home') {
      router.push('/financial')
    }
  }
}

const handleAppSelect = (index: string) => {
  // 验证角色与应用的匹配关系
  const currentRoleValue = currentRole.value
  let needsRoleChange = false
  
  if (index === '/credential' && !['education_institution', 'student', 'employer'].includes(currentRoleValue)) {
    ElMessage.warning('您当前角色不适用于学历凭证系统，已自动切换为教育机构角色')
    localStorage.setItem('role', 'education_institution')
    currentRole.value = 'education_institution'
    needsRoleChange = true
  } else if (index === '/financial' && !['financial_institution', 'individual', 'verifier'].includes(currentRoleValue)) {
    ElMessage.warning('您当前角色不适用于金融凭证系统，已自动切换为金融机构角色')
    localStorage.setItem('role', 'financial_institution')
    currentRole.value = 'financial_institution'
    needsRoleChange = true
  }
  
  // 导航逻辑，如果角色改变则添加延时
  console.log('Navigating to:', index)
  if (needsRoleChange) {
    setTimeout(() => {
      router.push(index)
    }, 100)
  } else {
    router.push(index)
  }
}
</script>

<style scoped>
.header {
  padding: 0;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid #eee;
  background-color: white;
}

.logo-container {
  padding: 0 20px;
  display: flex;
  align-items: center;
}

.logo {
  margin: 0;
  font-size: 20px;
  font-weight: bold;
  color: var(--el-color-primary);
}

.nav-container {
  flex: 1;
  display: flex;
  justify-content: center;
}

.app-nav {
  display: flex;
  align-items: center;
}

.role-selector {
  display: flex;
  align-items: center;
  padding: 0 20px;
}

.label {
  margin-right: 10px;
}
</style>
