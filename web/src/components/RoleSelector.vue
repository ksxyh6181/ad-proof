<template>
  <div class="role-selector">
    <el-dropdown @command="handleRoleChange" trigger="click">
      <span class="el-dropdown-link">
        <div class="role-display">
          <el-icon><User /></el-icon>
          当前角色: {{ roleLabel }}
          <el-icon class="el-icon--right"><arrow-down /></el-icon>
        </div>
      </span>
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item v-for="role in availableRoles" :key="role.value" :command="role.value">
            <el-icon v-if="role.icon"><component :is="role.icon" /></el-icon>
            {{ role.label }}
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { User, School, OfficeBuilding, Check, ArrowDown } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'

interface RoleOption {
  value: string
  label: string
  icon?: string
}

// 定义可用角色
const availableRoles = [
  { value: 'education_institution', label: '教育机构', icon: 'School' },
  { value: 'student', label: '学生', icon: 'User' },
  { value: 'verifier', label: '企业验证方', icon: 'Check' }
]

// 当前选择的角色
const currentRole = ref(localStorage.getItem('role') || 'education_institution')

// 根据角色值获取显示标签
const roleLabel = computed(() => {
  const role = availableRoles.find(r => r.value === currentRole.value)
  return role ? role.label : '未知角色'
})

// 处理角色变更
const handleRoleChange = (role: string) => {
  if (role === currentRole.value) return
  
  localStorage.setItem('role', role)
  currentRole.value = role
  ElMessage.success(`角色已切换为: ${availableRoles.find(r => r.value === role)?.label}`)
  
  // 刷新页面以应用新角色
  setTimeout(() => {
    window.location.reload()
  }, 300)
}

// 组件挂载时确保有默认角色
onMounted(() => {
  if (!localStorage.getItem('role')) {
    localStorage.setItem('role', 'education_institution')
    currentRole.value = 'education_institution'
  }
})
</script>

<style scoped>
.role-selector {
  display: inline-block;
  cursor: pointer;
}

.role-display {
  display: flex;
  align-items: center;
  padding: 4px 8px;
  background-color: rgba(64, 158, 255, 0.1);
  border-radius: 4px;
  color: #409eff;
  font-size: 14px;
  transition: all 0.3s;
}

.role-display:hover {
  background-color: rgba(64, 158, 255, 0.2);
}

.el-dropdown-link {
  cursor: pointer;
  display: flex;
  align-items: center;
}

.el-icon {
  margin-right: 4px;
}

.el-icon--right {
  margin-right: 0;
  margin-left: 4px;
}
</style>
