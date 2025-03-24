<template>
  <div class="get-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span class="title">获取证书</span>
          <el-tag v-if="currentRoleTag" :type="currentRoleTag.type" effect="dark">{{ currentRoleTag.text }}</el-tag>
        </div>
      </template>

      <div class="search-container">
        <el-form ref="formRef" :model="form" :rules="rules" label-width="120px">
          <el-form-item label="证书哈希" prop="hash">
            <el-input 
              v-model="form.hash" 
              placeholder="请输入证书哈希"
              @input="clearCredentialData"
              :suffix-icon="Search"
            />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" :loading="loading" @click="handleSearch">
              {{ loading ? '获取中...' : '获取证书' }}
            </el-button>
            <el-button @click="resetForm">重置</el-button>
          </el-form-item>
        </el-form>
      </div>

      <el-divider v-if="credential" content-position="center">证书信息</el-divider>

      <div v-if="credential" class="credential-details">
        <el-tabs v-model="activeTab" type="card">
          <el-tab-pane label="基本信息" name="basic">
            <el-descriptions :column="1" border class="descriptions-container">
              <el-descriptions-item label="学号">{{ credential.student_id }}</el-descriptions-item>
              <el-descriptions-item label="姓名">{{ credential.name }}</el-descriptions-item>
              <el-descriptions-item label="学位">{{ credential.degree }}</el-descriptions-item>
              <el-descriptions-item label="毕业日期">{{ credential.graduation_date }}</el-descriptions-item>
              <el-descriptions-item label="证书哈希">
                <el-tag size="small" type="info">{{ credential.hash }}</el-tag>
              </el-descriptions-item>
            </el-descriptions>
            
            <div class="actions-container">
              <el-button type="primary" @click="navigateToVerify(credential.hash)">
                验证此证书
              </el-button>
              <el-button type="success" @click="downloadCertificateData">
                下载证书数据
              </el-button>
            </div>
          </el-tab-pane>
          
          <el-tab-pane v-if="showTechnicalDetails" label="技术详情" name="technical">
            <div class="technical-details">
              <h4>公共输入 (Public Inputs)</h4>
              <el-input
                v-model="publicInputsString"
                type="textarea"
                :rows="3"
                readonly
                class="technical-input"
              />
              
              <h4>证明 (Proof)</h4>
              <el-input
                v-model="proofString"
                type="textarea"
                :rows="6"
                readonly
                class="technical-input"
              />
            </div>
          </el-tab-pane>
        </el-tabs>
      </div>

      <div v-if="error" class="error-container">
        <el-result
          icon="error"
          title="获取证书失败"
          :sub-title="error"
        >
          <template #extra>
            <el-button type="primary" @click="resetForm">重新查询</el-button>
          </template>
        </el-result>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { getAllCredentials } from '@/api/credential'
import type { Credential } from '@/types/credential';
import { Search } from '@element-plus/icons-vue'
import QRCode from 'qrcode';

const route = useRoute()
const router = useRouter()
const formRef = ref<FormInstance>()
const loading = ref(false)
const credential = ref<Credential | null>(null)
const error = ref<string | null>(null)
const activeTab = ref('basic')

const form = ref({
  hash: route.query.hash as string || ''
})

const rules = ref<FormRules>({
  hash: [
    { required: true, message: '请输入证书哈希', trigger: 'blur' },
    { min: 10, message: '哈希长度不能小于10个字符', trigger: 'blur' }
  ]
})

const currentRole = computed(() => localStorage.getItem('role') || '')

const currentRoleTag = computed(() => {
  const role = currentRole.value
  if (role === 'education_institution') {
    return { text: '教育机构', type: 'success' }
  } else if (role === 'student') {
    return { text: '学生视图', type: 'info' }
  } else if (role === 'verifier') {
    return { text: '企业验证方', type: 'warning' }
  }
  return null
})

// 确定是否显示技术详情标签页
const showTechnicalDetails = computed(() => {
  // 仅允许教育机构和验证方查看详细技术信息
  return ['education_institution', 'verifier'].includes(currentRole.value)
})

const publicInputsString = computed(() => {
  if (!credential.value?.public_inputs) return ''
  try {
    return JSON.stringify(credential.value.public_inputs, null, 2)
  } catch (e) {
    return String(credential.value.public_inputs)
  }
})

const proofString = computed(() => {
  if (!credential.value?.proof) return ''
  try {
    return JSON.stringify(credential.value.proof, null, 2)
  } catch (e) {
    return String(credential.value.proof)
  }
})

// 当URL参数hash存在时，自动搜索
watch(() => route.query.hash, (newHash) => {
  if (newHash) {
    form.value.hash = newHash as string
    handleSearch()
  }
}, { immediate: true })

const resetForm = () => {
  if (formRef.value) {
    formRef.value.resetFields()
  }
  credential.value = null
  error.value = null
}

const clearCredentialData = () => {
  credential.value = null
  error.value = null
}

const handleSearch = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid, fields) => {
    if (!valid) {
      console.log('表单验证失败:', fields)
      return
    }
    
    loading.value = true
    error.value = null
    credential.value = null
    
    try {
      const result = await getAllCredentials(form.value.hash)
      
      if (!result) {
        error.value = '未找到该证书'
        return
      }
      
      credential.value = result
      
      // 如果用户无权查看技术详情，自动切换到基本信息标签
      if (!showTechnicalDetails.value) {
        activeTab.value = 'basic'
      }
      
    } catch (e: any) {
      console.error('Get credential error:', e)
      error.value = e?.response?.data?.msg || e?.message || '获取证书失败'
    } finally {
      loading.value = false
    }
  })
}

const navigateToVerify = (hash: string) => {
  router.push({
    path: '/credential/verify',
    query: { hash }
  })
}

const downloadCertificateData = () => {
  if (!credential.value) return
  
  try {
    const data = JSON.stringify(credential.value, null, 2)
    const blob = new Blob([data], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    
    const link = document.createElement('a')
    link.href = url
    link.download = `certificate-${credential.value.hash?.substring(0, 8) || 'data'}.json`
    document.body.appendChild(link)
    link.click()
    document.body.removeChild(link)
    
    ElMessage.success('证书数据已下载')
  } catch (e) {
    console.error('Download error:', e)
    ElMessage.error('下载失败')
  }
}

onMounted(() => {
  // 如果URL中有哈希参数，自动执行搜索
  if (route.query.hash) {
    form.value.hash = route.query.hash as string
    handleSearch()
  }
})
</script>

<style scoped>
.get-container {
  padding: 20px;
  max-width: 900px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.title {
  font-size: 18px;
  font-weight: bold;
}

.credential-details {
  margin-top: 20px;
}

.descriptions-container {
  margin-bottom: 20px;
}

.actions-container {
  margin-top: 20px;
  display: flex;
  gap: 10px;
}

.technical-details {
  padding: 10px;
  background-color: #f9f9f9;
  border-radius: 4px;
}

.technical-details h4 {
  margin: 15px 0 5px;
  color: #606266;
}

.technical-input {
  margin-bottom: 15px;
  font-family: monospace;
}

.error-container {
  margin-top: 20px;
}
</style>
