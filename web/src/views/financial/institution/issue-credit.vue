<template>
  <div class="issue-credit-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <h2>颁发信用评分证明</h2>
          <p class="description">作为金融机构，您可以为用户颁发信用评分证明，该证明使用零知识证明技术保护用户隐私。</p>
        </div>
      </template>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="120px">
        <el-form-item label="个人ID" prop="personal_id">
          <el-input v-model="form.personal_id" placeholder="请输入个人ID（如税号）" />
        </el-form-item>
        <el-form-item label="信用评分" prop="credit_score">
          <el-slider
            v-model="form.credit_score"
            :min="300"
            :max="850"
            :step="1"
            :format-tooltip="formatTooltip"
            show-input
          />
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
          <el-button type="primary" @click="submitForm" :loading="loading">颁发信用评分证明</el-button>
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
          <el-descriptions-item label="信用评分范围">{{ resultCredential.credit_score_range }}</el-descriptions-item>
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
import { ref, reactive } from 'vue'
import { ElMessage, ElForm } from 'element-plus'
import type { FormInstance, FormRules } from 'element-plus'
import { financialApi } from '@/api/financial'
import type { FinancialCredential } from '@/types/financial'
import { useClipboard } from '@vueuse/core'

const formRef = ref<FormInstance>()
const { copy } = useClipboard()

const form = reactive({
  personal_id: '',
  credit_score: 700,
  issuer_id: '',
  expiry_date: ''
})

const rules = reactive<FormRules>({
  personal_id: [
    { required: true, message: '请输入个人ID', trigger: 'blur' },
    { min: 3, max: 30, message: '长度应为3到30个字符', trigger: 'blur' }
  ],
  credit_score: [
    { required: true, message: '请输入信用评分', trigger: 'change' }
  ],
  issuer_id: [
    { required: true, message: '请输入发行机构ID', trigger: 'blur' }
  ],
  expiry_date: [
    { required: true, message: '请选择过期日期', trigger: 'change' }
  ]
})

const formatTooltip = (val: number) => {
  let level = '低'
  if (val < 580) level = '较差'
  else if (val < 670) level = '一般'
  else if (val < 740) level = '良好'
  else if (val < 800) level = '优秀'
  else level = '极佳'
  
  return `${val} (${level})`
}

const loading = ref(false)
const dialogVisible = ref(false)
const resultCredential = ref<FinancialCredential | null>(null)

const submitForm = async () => {
  if (!formRef.value) return
  
  await formRef.value.validate(async (valid) => {
    if (valid) {
      loading.value = true
      try {
        const res = await financialApi.issueCreditScore(form)
        resultCredential.value = res.data?.data?.credential || null
        dialogVisible.value = true
        ElMessage.success('信用评分证明已成功颁发')
      } catch (error: any) {
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
.issue-credit-container {
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
