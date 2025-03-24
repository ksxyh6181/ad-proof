<template>
  <div class="verify-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span class="title">验证学历证书</span>
          <el-tag v-if="currentRoleTag" :type="currentRoleTag.type" effect="dark">{{ currentRoleTag.text }}</el-tag>
        </div>
      </template>

      <div class="form-container">
        <el-form v-if="!verificationResult" ref="formRef" :model="form" :rules="rules" label-width="120px">
          <el-form-item label="证书哈希" prop="hash">
            <el-input 
              v-model="form.hash" 
              placeholder="请输入证书哈希值"
              @input="clearFetchedData" 
              :suffix-icon="Search"
            />
          </el-form-item>

          <el-form-item>
            <el-button type="primary" :loading="fetchingData" @click="fetchCredential">
              获取证书数据
            </el-button>
          </el-form-item>

          <el-divider v-if="fetchedCredential">证书数据</el-divider>

          <template v-if="fetchedCredential">
            <div class="credential-info">
              <el-descriptions :column="1" border>
                <el-descriptions-item label="学号">{{ fetchedCredential.student_id }}</el-descriptions-item>
                <el-descriptions-item label="姓名">{{ fetchedCredential.name }}</el-descriptions-item>
                <el-descriptions-item label="学位">{{ fetchedCredential.degree }}</el-descriptions-item>
                <el-descriptions-item label="毕业日期">{{ fetchedCredential.graduation_date }}</el-descriptions-item>
              </el-descriptions>
            </div>

            <div class="verification-actions">
              <el-button type="success" :loading="verifying" @click="verifyCredential">
                {{ verifying ? '验证中...' : '验证证书' }}
              </el-button>
              <el-button @click="resetForm">取消</el-button>
            </div>
          </template>
        </el-form>

        <div v-else class="verification-result">
          <el-result
            :icon="verificationResult.success ? 'success' : 'error'"
            :title="verificationResult.title"
            :sub-title="verificationResult.message"
          >
            <template #extra>
              <el-button type="primary" @click="resetForm">继续验证</el-button>
            </template>
          </el-result>
        </div>
      </div>
    </el-card>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { verifyCredential } from '@/api/credential'
import type { CredentialVerifyParams, Credential } from '@/types/credential'
import { Search } from '@element-plus/icons-vue'

const formRef = ref<FormInstance>()
const fetchingData = ref(false)
const verifying = ref(false)
const fetchedCredential = ref<Credential | null>(null)
const verificationResult = ref<{
  success: boolean;
  title: string;
  message: string;
} | null>(null)

const form = ref<CredentialVerifyParams>({
  hash: '',
  content: ''
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

const resetForm = () => {
  if (formRef.value) {
    formRef.value.resetFields()
  }
  fetchedCredential.value = null
  verificationResult.value = null
}

const clearFetchedData = () => {
  fetchedCredential.value = null
  verificationResult.value = null
}

const fetchCredential = async () => {
  if (!formRef.value) return
  
  await formRef.value.validateField('hash', async (valid) => {
    if (!valid) return
    
    fetchingData.value = true
    try {
      const result = await verifyCredential({ hash: form.value.hash })
      
      if (!result) {
        ElMessage.error('未找到该证书')
        return
      }
      
      fetchedCredential.value = result.credential
      
    } catch (error: any) {
      console.error('Fetch credential error:', error)
      ElMessage.error(error?.response?.data?.msg || error?.message || '获取证书失败')
    } finally {
      fetchingData.value = false
    }
  })
}

const verifyCredential = async () => {
  if (!fetchedCredential.value) {
    ElMessage.error('缺少必要的证书数据，无法验证')
    return
  }
  
  verifying.value = true
  try {
    const result = await verifyCredential({ hash: form.value.hash, content: form.value.content })
    
    verificationResult.value = {
      success: result.valid,
      title: result.valid ? '证书验证成功' : '证书验证失败',
      message: result.valid ? '该证书是有效且真实的' : '该证书是无效或伪造的'
    }
    
  } catch (error: any) {
    console.error('Verify credential error:', error)
    
    verificationResult.value = {
      success: false,
      title: '证书验证失败',
      message: error?.response?.data?.msg || error?.message || '验证过程发生错误'
    }
  } finally {
    verifying.value = false
  }
}
</script>

<style scoped>
.verify-container {
  padding: 20px;
  max-width: 800px;
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

.credential-info {
  margin: 20px 0;
}

.verification-actions {
  margin-top: 20px;
  display: flex;
  justify-content: flex-start;
  gap: 10px;
}

.verification-result {
  margin-top: 20px;
}
</style>
