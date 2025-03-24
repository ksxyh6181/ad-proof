<template>
  <div class="issue-income-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <h2>颁发收入证明</h2>
          <p class="description">作为金融机构，您可以为用户颁发收入证明，该证明使用零知识证明技术保护用户隐私。</p>
        </div>
      </template>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="120px">
        <el-form-item label="个人ID" prop="personal_id">
          <el-input v-model="form.personal_id" placeholder="请输入个人ID（如税号）" />
        </el-form-item>
        <el-form-item label="实际收入" prop="actual_income">
          <el-input-number v-model="form.actual_income" :precision="2" :step="1000" :min="0" style="width: 100%;" />
        </el-form-item>
        <el-form-item label="发行机构ID" prop="issuer_id">
          <el-input v-model="form.issuer_id" placeholder="请输入发行机构ID" />
        </el-form-item>
        <el-form-item label="过期日期" prop="expiry_date">
          <el-date-picker
            v-model="form.expiry_date"
            type="date"
            placeholder="选择过期日期"
            style="width: 100%"
            value-format="YYYY-MM-DD"
          />
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitForm" :loading="loading">颁发收入证明</el-button>
          <el-button @click="resetForm">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 结果显示 -->
    <el-dialog v-model="dialogVisible" title="凭证颁发成功" width="50%">
      <div v-if="resultCredential">
        <el-alert
          title="凭证已成功颁发"
          type="success"
          description="请妥善保存以下凭证信息，特别是哈希值，它是验证凭证的唯一标识。"
          :closable="false"
          show-icon
        />
        <el-descriptions title="凭证详情" :column="1" border>
          <el-descriptions-item label="个人ID">{{ resultCredential.personal_id }}</el-descriptions-item>
          <el-descriptions-item label="收入等级">{{ resultCredential.income_level }}</el-descriptions-item>
          <el-descriptions-item label="凭证类型">{{ resultCredential.credential_type }}</el-descriptions-item>
          <el-descriptions-item label="发行机构">{{ resultCredential.issuer_id }}</el-descriptions-item>
          <el-descriptions-item label="发行日期">{{ resultCredential.issue_date }}</el-descriptions-item>
          <el-descriptions-item label="过期日期">{{ resultCredential.expiry_date }}</el-descriptions-item>
          <el-descriptions-item label="凭证哈希">
            <el-tag size="small" type="info">{{ resultCredential.hash }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">关闭</el-button>
          <el-button type="primary" @click="copyHash">复制哈希</el-button>
        </span>
      </template>
    </el-dialog>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElForm } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { financialApi } from '@/api/financial'
import type { FinancialCredential } from '@/types/financial'
import { useClipboard } from '@vueuse/core'

const formRef = ref<FormInstance>()
const { copy } = useClipboard()

// 计算一年后的日期作为默认过期日期
const getOneYearLater = () => {
  const date = new Date()
  date.setFullYear(date.getFullYear() + 1)
  return date.toISOString().split('T')[0] // 格式为YYYY-MM-DD
}

const form = reactive({
  personal_id: '',
  actual_income: 5000,
  issuer_id: 'financial_institution_001', // 设置默认值
  expiry_date: ''
})

onMounted(() => {
  // 在组件挂载时设置默认过期日期
  form.expiry_date = getOneYearLater()
})

const rules = reactive<FormRules>({
  personal_id: [
    { required: true, message: '请输入个人ID', trigger: 'blur' },
    { min: 3, max: 30, message: '长度应为3到30个字符', trigger: 'blur' }
  ],
  actual_income: [
    { required: true, message: '请输入实际收入', trigger: 'change' }
  ],
  issuer_id: [
    { required: true, message: '请输入发行机构ID', trigger: 'blur' }
  ],
  expiry_date: [
    { required: true, message: '请选择过期日期', trigger: 'change' }
  ]
})

const loading = ref(false)
const dialogVisible = ref(false)
const resultCredential = ref<FinancialCredential | null>(null)

const submitForm = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        // 确保personal_id不为空且格式正确
        if (!form.personal_id.trim()) {
          form.personal_id = `user_${Math.floor(Math.random() * 10000)}`
        }
        
        // 确保actual_income为正数
        if (form.actual_income <= 0) {
          form.actual_income = 5000
        }
        
        // 确保issuer_id不为空
        if (!form.issuer_id.trim()) {
          form.issuer_id = 'financial_institution_001'
        }
        
        // 确保过期日期有效
        if (!form.expiry_date) {
          form.expiry_date = getOneYearLater()
        }
        
        console.log('预处理后的表单数据:', form)
        
        // 调用后端API
        const response = await financialApi.issueIncomeProof({
          personal_id: form.personal_id.trim(),
          actual_income: form.actual_income,
          issuer_id: form.issuer_id.trim(),
          expiry_date: form.expiry_date
        })
        
        console.log('Response from server:', response)
        
        // 处理响应数据结构
        if (response && response.data && response.data.credential) {
          resultCredential.value = response.data.credential
        } else if (response && response.credential) {
          resultCredential.value = response.credential
        } else if (response && response.hash) {
          // 如果只有哈希返回，手动构建凭证对象
          resultCredential.value = {
            personal_id: form.personal_id,
            credential_type: 'income',
            issuer_id: form.issuer_id,
            issue_date: new Date().toISOString().split('T')[0],
            expiry_date: form.expiry_date,
            hash: response.hash,
            income_level: determineIncomeLevel(form.actual_income)
          }
        } else {
          throw new Error('无法获取凭证信息')
        }
        
        dialogVisible.value = true
        ElMessage.success('收入证明已成功颁发')
      } catch (error: any) {
        console.error('Error submitting form:', error)
        ElMessage.error(error.message || '颁发失败，请重试')
      } finally {
        loading.value = false
      }
    } else {
      ElMessage.warning('请完成表单填写')
      return false
    }
  })
}

// 根据收入金额确定收入等级
const determineIncomeLevel = (income: number): string => {
  if (income < 5000) return 'level_1'
  if (income < 10000) return 'level_2'
  if (income < 20000) return 'level_3'
  if (income < 50000) return 'level_4'
  return 'level_5'
}

const resetForm = () => {
  formRef.value?.resetFields()
}

const copyHash = () => {
  if (resultCredential.value?.hash) {
    copy(resultCredential.value.hash)
    ElMessage.success('凭证哈希已复制到剪贴板')
  }
}
</script>

<style scoped>
.issue-income-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

.card-header {
  margin-bottom: 20px;
}

.description {
  color: #606266;
  font-size: 14px;
  margin-top: 8px;
}
</style>
