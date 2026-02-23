<template>
  <div class="verify-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <h2>验证金融凭证</h2>
          <p class="description">验证用户提供的金融凭证，无需获取用户的敏感个人信息。</p>
        </div>
      </template>
      <el-form ref="formRef" :model="form" :rules="rules" label-width="120px">
        <el-form-item label="凭证哈希" prop="hash">
          <el-input v-model="form.hash" placeholder="请输入凭证哈希值" />
        </el-form-item>
        
        <el-form-item label="验证要求" prop="requirementType">
          <el-select v-model="form.requirementType" placeholder="请选择您的验证要求" style="width: 100%">
            <el-option label="验证收入等级" value="income" />
            <el-option label="验证信用评分" value="credit" />
            <el-option label="验证跨境信用" value="cross_border" />
            <el-option label="仅验证凭证有效性" value="validity" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="最低收入等级" prop="minIncomeLevel" v-if="form.requirementType === 'income'">
          <el-select v-model="form.minIncomeLevel" placeholder="请选择最低收入等级要求" style="width: 100%">
            <el-option label="无要求" value="" />
            <el-option label="低收入及以上 (≥¥5,000)" value="level_1" />
            <el-option label="中低收入及以上 (≥¥10,000)" value="level_2" />
            <el-option label="中等收入及以上 (≥¥20,000)" value="level_3" />
            <el-option label="中高收入及以上 (≥¥50,000)" value="level_4" />
            <el-option label="高收入 (>¥50,000)" value="level_5" />
          </el-select>
        </el-form-item>
        
        <el-form-item label="最低信用等级" prop="minCreditRange" v-if="form.requirementType === 'credit'">
          <el-select v-model="form.minCreditRange" placeholder="请选择最低信用等级要求" style="width: 100%">
            <el-option label="无要求" value="" />
            <el-option label="一般及以上 (≥580)" value="fair" />
            <el-option label="良好及以上 (≥670)" value="good" />
            <el-option label="优秀及以上 (≥740)" value="very_good" />
            <el-option label="仅极佳 (≥800)" value="excellent" />
          </el-select>
        </el-form-item>
        
        <el-form-item>
          <el-button type="primary" @click="verifyCredential" :loading="loading">验证凭证</el-button>
          <el-button @click="resetForm">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 验证结果 -->
    <div v-if="verificationResult" class="verification-result mt-4">
      <el-alert
        :title="verificationResult.is_valid 
          ? '✅ 凭证有效' 
          : '❌ 凭证无效'"
        :type="verificationResult.is_valid ? 'success' : 'error'"
        show-icon
      >
        <template #default>
          <p>{{ verificationResult.message || (verificationResult.is_valid ? '该凭证已通过验证' : '验证失败') }}</p>
          
          <p v-if="verificationResult.is_valid && form.requirementType !== 'validity'">
            要求检查: <strong>{{ requirementMet ? '满足要求 ✓' : '不满足要求 ✗' }}</strong>
          </p>
        </template>
      </el-alert>

      <!-- 凭证详情 -->
      <div v-if="showCredentialDetails" class="credential-details mt-4">
        <h3>凭证详情</h3>
        <el-descriptions :column="1" border>
          <el-descriptions-item label="持有人ID">{{ credentialDetails.personal_id }}</el-descriptions-item>
          <el-descriptions-item label="凭证类型">{{ credentialTypes[credentialDetails.credential_type] || credentialDetails.credential_type }}</el-descriptions-item>
          <el-descriptions-item label="发行机构">{{ credentialDetails.issuer_id }}</el-descriptions-item>
          <el-descriptions-item label="发行日期">{{ credentialDetails.issue_date }}</el-descriptions-item>
          <el-descriptions-item label="过期日期">{{ credentialDetails.expiry_date }}</el-descriptions-item>
          <el-descriptions-item label="凭证哈希">{{ credentialDetails.hash }}</el-descriptions-item>
          
          <el-descriptions-item v-if="credentialDetails.income_level" label="收入等级">
            {{ incomeLevels[credentialDetails.income_level] || credentialDetails.income_level }}
          </el-descriptions-item>
          
          <el-descriptions-item v-if="credentialDetails.credit_score_range" label="信用评分区间">
            {{ creditRanges[credentialDetails.credit_score_range] || credentialDetails.credit_score_range }}
          </el-descriptions-item>
          
          <el-descriptions-item v-if="credentialDetails.mock" label="备注">
            <el-tag type="warning">前端生成凭证</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { financialApi } from '@/api/financial'

const formRef = ref<FormInstance>()

const form = reactive({
  hash: '',
  requirementType: 'validity',
  minIncomeLevel: '',
  minCreditRange: ''
})

const rules = reactive<FormRules>({
  hash: [
    { required: true, message: '请输入凭证哈希', trigger: 'blur' },
    { min: 10, message: '哈希长度至少为10个字符', trigger: 'blur' }
  ],
  requirementType: [
    { required: true, message: '请选择验证要求', trigger: 'change' }
  ]
})

const loading = ref(false)
const verificationDone = ref(false)
const verificationResult = ref<{
  is_valid: boolean,
  message?: string,
  credential?: any,
  error?: string
} | null>(null)

const showCredentialDetails = computed(() => {
  return verificationResult.value && verificationResult.value.is_valid && verificationResult.value.credential;
})

const credentialDetails = computed(() => {
  if (!verificationResult.value || !verificationResult.value.credential) return null;
  return verificationResult.value.credential;
})

const requirementMet = computed(() => {
  if (!verificationResult.value || !verificationResult.value.is_valid) return false;
  
  const credential = verificationResult.value.credential;
  if (!credential) return false;
  
  if (form.requirementType === 'validity') {
    // 仅验证有效性
    return true;
  } else if (form.requirementType === 'income' && form.minIncomeLevel && credential.income_level) {
    // 验证收入等级
    const credentialLevel = incomeLevelOrder[credential.income_level] || 0;
    const requiredLevel = incomeLevelOrder[form.minIncomeLevel] || 0;
    return credentialLevel >= requiredLevel;
  } else if (form.requirementType === 'credit' && form.minCreditRange && credential.credit_score_range) {
    // 验证信用评分
    const credentialRange = creditRangeOrder[credential.credit_score_range] || 0;
    const requiredRange = creditRangeOrder[form.minCreditRange] || 0;
    return credentialRange >= requiredRange;
  } else if (form.requirementType === 'cross_border') {
    // 验证跨境信用
    return credential.credential_type === 'cross_border';
  }
  
  // 如果我们无法获取所需的特定字段，但凭证有效，则视为满足要求
  return true;
})

const incomeLevelOrder: Record<string, number> = {
  'level_1': 1,
  'level_2': 2,
  'level_3': 3,
  'level_4': 4,
  'level_5': 5
};

const creditRangeOrder: Record<string, number> = {
  'poor': 1,
  'fair': 2,
  'good': 3,
  'very_good': 4,
  'excellent': 5
};

// 凭证类型文本映射
const credentialTypes: Record<string, string> = {
  'income': '收入证明',
  'credit': '信用评分证明',
  'cross_border': '跨境信用证明'
};

// 收入等级文本映射
const incomeLevels: Record<string, string> = {
  'level_1': '低收入 (< ¥5,000)',
  'level_2': '中低收入 (¥5,000-¥10,000)',
  'level_3': '中等收入 (¥10,000-¥20,000)',
  'level_4': '中高收入 (¥20,000-¥50,000)',
  'level_5': '高收入 (> ¥50,000)'
};

// 信用评分区间文本映射
const creditRanges: Record<string, string> = {
  'poor': '较差 (< 600)',
  'fair': '一般 (600-650)',
  'good': '良好 (650-700)',
  'very_good': '优秀 (700-750)',
  'excellent': '极好 (> 750)'
};

const verifyCredential = async () => {
  if (!formRef.value) return
  
  loading.value = true
  
  try {
    // 使用hash作为查询参数
    let hash = form.hash || ''
    
    console.log('提交验证请求：', { hash })
    
    // 添加详细的调试信息
    try {
      // 尝试使用API进行验证
      const response = await financialApi.verifyFinancial({ hash })
      console.log('验证API响应:', response)
      
      // 检查响应结构
      if (!response || !response.data) {
        console.error('API响应缺少数据部分')
        verificationResult.value = {
          is_valid: false,
          message: '验证失败：服务器返回了无效的响应',
          error: '无效API响应'
        }
        return
      }
      
      // 处理API响应
      if (response.data.success === false) {
        console.error('验证失败，服务器返回错误:', response.data.data)
        verificationResult.value = {
          is_valid: false,
          message: response.data.data || '验证失败',
          error: response.data.data
        }
      } else {
        console.log('验证成功，获取凭证详情')
        // 如果验证成功，尝试获取凭证详情
        try {
          const credentialData = await financialApi.getFinancial(hash)
          console.log('凭证详情API响应:', credentialData)
          
          if (credentialData && credentialData.data && credentialData.data.success) {
            const credential = credentialData.data.data
            console.log('解析出的凭证数据:', credential)
            verificationResult.value = {
              is_valid: true,
              message: response.data.data?.message || '凭证验证通过',
              credential: credential
            }
          } else {
            console.warn('获取凭证详情成功，但响应不符合预期:', credentialData)
            verificationResult.value = {
              is_valid: true,
              message: response.data.data?.message || '凭证验证通过，但无法获取详情',
              credential: null
            }
          }
        } catch (detailError) {
          console.error('获取凭证详情失败:', detailError)
          verificationResult.value = {
            is_valid: true,
            message: response.data.data?.message || '凭证验证通过，但无法获取详情',
            credential: null
          }
        }
      }
    } catch (apiError: any) {
      console.error('验证API调用失败:', apiError)
      console.error('API错误详情:', apiError.response || apiError.message || apiError)
      
      verificationResult.value = {
        is_valid: false,
        message: '验证API调用失败',
        error: apiError.response?.data?.data || apiError.message || '未知错误'
      }
    }
  } catch (e: any) {
    console.error('验证过程中发生错误:', e)
    verificationResult.value = {
      is_valid: false,
      message: '验证过程中发生错误',
      error: e.message || '未知错误'
    }
  } finally {
    loading.value = false
  }
}

const resetForm = () => {
  formRef.value?.resetFields();
  verificationDone.value = false;
  verificationResult.value = null;
};
</script>

<style scoped>
.verify-container {
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

.result-card {
  margin-top: 20px;
}

.result-details {
  margin-top: 20px;
}

.privacy-note {
  margin-top: 20px;
}
</style>
