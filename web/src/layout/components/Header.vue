<template>
  <div class="header-shell">
    <button class="brand" type="button" @click="router.push('/home')">
      <span class="brand-mark">AP</span>
      <div class="brand-copy">
        <strong>Ad Proof</strong>
        <span>Selective Disclosure VC Demo</span>
      </div>
    </button>

    <nav class="nav">
      <button
        v-for="item in navItems"
        :key="item.path"
        class="nav-link"
        :class="{ active: activePath === item.path }"
        type="button"
        @click="router.push(item.path)"
      >
        {{ item.label }}
      </button>
    </nav>

    <div class="role-panel">
      <span class="role-label">当前请求角色</span>
      <el-select v-model="currentRole" size="large" @change="handleRoleChange">
        <el-option
          v-for="role in roles"
          :key="role.value"
          :label="role.label"
          :value="role.value"
        />
      </el-select>
      <small class="role-note">签发接口校验角色，生成与验证接口对角色开放。</small>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { ElMessage } from 'element-plus'
import { DEMO_ROLES, getStoredRole, setStoredRole, type DemoRole } from '@/utils/role'

const route = useRoute()
const router = useRouter()

const navItems = [
  { path: '/home', label: '首页' },
  { path: '/income', label: '收入门槛证明' },
  { path: '/kyc', label: 'KYC 等级证明' }
]

const roles = DEMO_ROLES
const currentRole = ref<DemoRole>(getStoredRole())
const activePath = computed(() => {
  if (route.path.startsWith('/income')) {
    return '/income'
  }
  if (route.path.startsWith('/kyc')) {
    return '/kyc'
  }
  return '/home'
})

const syncRole = () => {
  currentRole.value = getStoredRole()
}

onMounted(() => {
  syncRole()
  window.addEventListener('storage', syncRole)
  window.addEventListener('role-change', syncRole as EventListener)
})

onBeforeUnmount(() => {
  window.removeEventListener('storage', syncRole)
  window.removeEventListener('role-change', syncRole as EventListener)
})

const handleRoleChange = (value: DemoRole) => {
  setStoredRole(value)
  const label = roles.find((role) => role.value === value)?.label ?? value
  ElMessage.success(`已切换到${label}`)
}
</script>

<style scoped>
.header-shell {
  height: 72px;
  width: min(1240px, calc(100% - 24px));
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 10px 0;
}

.brand {
  display: inline-flex;
  align-items: center;
  gap: 12px;
  border: none;
  background: transparent;
  padding: 0;
}

.brand-mark {
  width: 42px;
  height: 42px;
  border-radius: 14px;
  display: grid;
  place-items: center;
  font-weight: 700;
  color: #fff8e8;
  background: linear-gradient(135deg, #c46d2d, #386c67);
  box-shadow: 0 12px 30px rgba(56, 108, 103, 0.18);
}

.brand-copy {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 2px;
}

.brand-copy strong {
  font-size: 1rem;
  color: var(--ap-text-strong);
}

.brand-copy span {
  font-size: 0.74rem;
  color: var(--ap-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.nav {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.nav-link {
  padding: 10px 16px;
  border-radius: 999px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--ap-text-muted);
}

.nav-link.active {
  color: var(--ap-text-strong);
  background: rgba(255, 248, 232, 0.9);
  border-color: rgba(196, 109, 45, 0.14);
  box-shadow: var(--ap-shadow-soft);
}

.role-panel {
  min-width: 260px;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 6px;
}

.role-label {
  font-size: 0.82rem;
  color: var(--ap-text-muted);
}

.role-note {
  color: var(--ap-text-soft);
  line-height: 1.45;
}

@media (max-width: 980px) {
  .header-shell {
    height: auto;
    flex-wrap: wrap;
    justify-content: center;
    padding: 12px 0;
  }

  .nav {
    order: 3;
    width: 100%;
    justify-content: flex-start;
    overflow-x: auto;
    padding-bottom: 4px;
  }

  .role-panel {
    min-width: 220px;
  }
}

@media (max-width: 640px) {
  .brand-copy span,
  .role-note {
    display: none;
  }

  .role-panel {
    width: 100%;
  }

  .nav-link {
    white-space: nowrap;
  }
}
</style>
