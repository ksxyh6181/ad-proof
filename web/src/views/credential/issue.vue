<template>
  <div class="issue-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span class="title">颁发学历证书</span>
          <el-tag v-if="showRoleInfo" type="success" effect="dark">教育机构专属</el-tag>
        </div>
      </template>

      <div class="back-navigation">
        <el-button class="go-back-button" @click="navigateToCredentialHome">
          <el-icon><ArrowLeft /></el-icon>返回证书首页
        </el-button>
      </div>

      <div v-if="!isEducationInstitution" class="role-warning">
        <el-alert
          title="权限提示"
          type="warning"
          description="只有教育机构角色才能颁发证书。请在顶部切换为教育机构角色。"
          show-icon
          :closable="false"
        />
      </div>

      <div class="navigation-menu" v-if="!successResult">
        <el-menu mode="horizontal" :router="true" class="credential-menu">
          <el-menu-item index="/credential/issue">颁发证书</el-menu-item>
          <el-menu-item index="/credential/verify">验证证书</el-menu-item>
          <el-menu-item index="/credential/get">查询证书</el-menu-item>
        </el-menu>
      </div>

      <el-form
        v-if="!successResult && isEducationInstitution"
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="120px"
        class="credential-form"
      >
        <el-form-item label="学号" prop="student_id">
          <el-input v-model="form.student_id" placeholder="请输入学号">
            <template #prefix>
              <el-icon><UserFilled /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="姓名" prop="name">
          <el-input v-model="form.name" placeholder="请输入姓名">
            <template #prefix>
              <el-icon><Avatar /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="学位" prop="degree">
          <el-select v-model="form.degree" placeholder="请选择学位" style="width: 100%">
            <el-option label="学士学位" value="学士学位" />
            <el-option label="硕士学位" value="硕士学位" />
            <el-option label="博士学位" value="博士学位" />
            <el-option label="专科学位" value="专科学位" />
          </el-select>
        </el-form-item>
        <el-form-item label="专业" prop="major">
          <el-input v-model="form.major" placeholder="请输入专业">
            <template #prefix>
              <el-icon><Notebook /></el-icon>
            </template>
          </el-input>
        </el-form-item>
        <el-form-item label="毕业日期" prop="graduation_date">
          <el-date-picker
            v-model="form.graduation_date"
            type="date"
            placeholder="选择日期"
            format="YYYY-MM-DD"
            value-format="YYYY-MM-DD"
            style="width: 100%"
          >
            <template #prefix>
              <el-icon><Calendar /></el-icon>
            </template>
          </el-date-picker>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" :loading="loading" @click="handleSubmit">
            {{ loading ? '提交中...' : '颁发证书' }}
          </el-button>
          <el-button @click="resetForm">重置</el-button>
        </el-form-item>
      </el-form>

      <div v-if="successResult" class="success-result">
        <el-result
          icon="success"
          title="证书颁发成功"
          sub-title="证书已经成功颁发并记录在系统中"
        >
          <template #extra>
            <div class="hash-display">
              <p><strong>证书哈希:</strong> {{ successResult.hash }}</p>
            </div>
            <div class="actions-container">
              <el-button type="primary" @click="resetForm">继续颁发</el-button>
              <el-button type="success" @click="navigateToGet(successResult.hash)">查看证书</el-button>
              <el-button type="warning" @click="navigateToVerify(successResult.hash)">验证证书</el-button>
              <el-button type="info" @click="copyHash">复制哈希值</el-button>
              <el-button type="primary" @click="navigateToCredentialHome">返回证书首页</el-button>
            </div>
          </template>
        </el-result>
      </div>
    </el-card>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import type { FormInstance, FormRules } from 'element-plus'
import { ElMessage } from 'element-plus'
import { credentialApi } from '../../api/credential'
import type { CredentialData } from '../../types/credential'
import { UserFilled, Avatar, Notebook, Calendar, ArrowLeft, School } from '@element-plus/icons-vue'

const router = useRouter()
const formRef = ref<FormInstance>()
const loading = ref(false)
const successResult = ref<{ hash: string } | null>(null)
const dialogVisible = ref(false)

// 添加专业字段
const form = ref<CredentialData & { major: string }>({
  student_id: '',
  name: '',
  degree: '',
  graduation_date: '',
  major: ''
})

const rules = ref<FormRules>({
  student_id: [
    { required: true, message: '请输入学号', trigger: 'blur' },
    { min: 3, max: 20, message: '长度在 3 到 20 个字符', trigger: 'blur' }
  ],
  name: [
    { required: true, message: '请输入姓名', trigger: 'blur' },
    { min: 2, max: 20, message: '长度在 2 到 20 个字符', trigger: 'blur' }
  ],
  degree: [
    { required: true, message: '请选择学位', trigger: 'change' }
  ],
  major: [
    { required: true, message: '请输入专业', trigger: 'blur' }
  ],
  graduation_date: [
    { required: true, message: '请选择毕业日期', trigger: 'change' }
  ]
})

const currentRole = computed(() => localStorage.getItem('role') || '')
const isEducationInstitution = computed(() => currentRole.value === 'education_institution')
const showRoleInfo = computed(() => isEducationInstitution.value)

const resetForm = () => {
  if (!formRef.value) return
  formRef.value.resetFields()
  successResult.value = null
}

const navigateToGet = (hash: string) => {
  router.push({
    path: '/credential/get',
    query: { hash }
  })
}

const navigateToVerify = (hash: string) => {
  router.push({
    path: '/credential/verify',
    query: { hash }
  })
}

const navigateToCredentialHome = () => {
  router.push('/credential')
}

const copyHash = () => {
  if (successResult.value && successResult.value.hash) {
    navigator.clipboard.writeText(successResult.value.hash)
      .then(() => {
        ElMessage.success('哈希值已复制到剪贴板')
      })
      .catch(err => {
        ElMessage.error('复制失败: ' + err)
      })
  }
}

const handleSubmit = async (event: Event) => {
  if (!formRef.value) return
  
  if (!isEducationInstitution.value) {
    ElMessage.error('只有教育机构角色才能颁发证书')
    return
  }
  
  await formRef.value.validate(async (valid, fields) => {
    if (!valid) {
      console.log('表单验证失败:', fields)
      return
    }
    
    loading.value = true
    try {
      // 整合专业信息到学位字段
      const submitData = {
        ...form.value,
        degree: `${form.value.degree} - ${form.value.major}`
      }
      
      // 排除major字段
      const { major, ...dataToSubmit } = submitData
      
      console.log('提交数据:', dataToSubmit)
      const result = await credentialApi.issue(dataToSubmit)
      console.log('服务器响应:', result)
      
      ElMessage.success('证书颁发成功')
      successResult.value = { hash: result.hash }
      dialogVisible.value = true
    } catch (error: any) {
      console.error('Issue credential error:', error)
      ElMessage.error(error?.response?.data?.msg || error?.message || '操作失败')
    } finally {
      loading.value = false
    }
  })
}
</script>

<style scoped>
.issue-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.back-navigation {
  margin-bottom: 16px;
}

.go-back-button {
  display: flex;
  align-items: center;
  font-size: 14px;
}

.go-back-button .el-icon {
  margin-right: 4px;
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

.description {
  margin: 8px 0;
  color: #606266;
  font-size: 14px;
  line-height: 1.5;
}

.role-warning {
  margin-bottom: 20px;
}

.navigation-menu {
  margin: 16px 0 24px 0;
}

.credential-menu {
  background-color: transparent;
  border-bottom: 1px solid #e6e6e6;
}

.credential-form {
  margin-top: 20px;
}

.success-result {
  margin-top: 20px;
}

.hash-display {
  padding: 10px;
  background-color: #f8f9fa;
  border-radius: 4px;
  margin: 16px 0;
  word-break: break-all;
  font-family: monospace;
}

.actions-container {
  display: flex;
  justify-content: center;
  gap: 10px;
  flex-wrap: wrap;
  margin-top: 16px;
}

.mt-4 {
  margin-top: 16px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
