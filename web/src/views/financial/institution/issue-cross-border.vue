<template>
  <div class="issue-cross-border-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <h2>颁发跨境信用证明</h2>
          <p class="description">为国际学生和移民提供跨境信用证明，帮助他们在新环境中获得金融支持。</p>
        </div>
      </template>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="120px">
        <el-form-item label="个人ID" prop="personal_id">
          <el-input v-model="form.personal_id" placeholder="请输入个人ID（如护照号）" />
        </el-form-item>
        
        <el-form-item label="收入等级" prop="income_level">
          <el-select v-model="form.income_level" placeholder="请选择收入等级" style="width: 100%">
            <el-option label="低收入 (低于$30,000)" value="level_1" />
            <el-option label="中低收入 ($30,000-$60,000)" value="level_2" />
            <el-option label="中等收入 ($60,000-$100,000)" value="level_3" />
            <el-option label="中高收入 ($100,000-$150,000)" value="level_4" />
            <el-option label="高收入 (高于$150,000)" value="level_5" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="信用评分范围" prop="credit_score_range">
          <el-select v-model="form.credit_score_range" placeholder="请选择信用评分范围" style="width: 100%">
            <el-option label="较差 (300-579)" value="poor" />
            <el-option label="一般 (580-669)" value="fair" />
            <el-option label="良好 (670-739)" value="good" />
            <el-option label="优秀 (740-799)" value="very_good" />
            <el-option label="极佳 (800-850)" value="excellent" />
          </el-select>
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
          <el-button type="primary" @click="submitForm" :loading="loading">颁发跨境信用证明</el-button>
          <el-button @click="resetForm">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 结果显示 -->
    <el-dialog v-model="dialogVisible" title="凭证颁发成功" width="50%">
      <div v-if="resultCredential">
        <el-alert
          title="跨境信用凭证已成功颁发"
          type="success"
          description="请妥善保存以下凭证信息，特别是哈希值，它是验证凭证的唯一标识。"
          :closable="false"
          show-icon
        />
        <el-descriptions title="凭证详情" :column="1" border>
          <el-descriptions-item label="个人ID">{{ resultCredential.personal_id }}</el-descriptions-item>
          <el-descriptions-item label="收入等级">{{ resultCredential.income_level }}</el-descriptions-item>
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
  income_level: 'level_3',
  credit_score_range: 'good',
  issuer_id: '',
  expiry_date: ''
})

const rules = reactive<FormRules>({
  personal_id: [
    { required: true, message: '请输入个人ID', trigger: 'blur' },
    { min: 3, max: 30, message: '长度应为3到30个字符', trigger: 'blur' }
  ],
  income_level: [
    { required: true, message: '请选择收入等级', trigger: 'change' }
  ],
  credit_score_range: [
    { required: true, message: '请选择信用评分范围', trigger: 'change' }
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
        const res = await financialApi.issueCrossBorder(form)
        resultCredential.value = res.data?.data?.credential || null
        dialogVisible.value = true
        ElMessage.success('跨境信用证明已成功颁发')
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
.issue-cross-border-container {
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
