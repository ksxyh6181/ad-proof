<template>
  <div class="my-credentials-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <h2>我的金融凭证</h2>
          <p class="description">查看和管理您的所有金融凭证，您可以随时选择向验证方提供这些凭证。</p>
        </div>
      </template>
      
      <div class="credential-actions">
        <el-input
          v-model="searchQuery"
          placeholder="搜索凭证..."
          prefix-icon="el-icon-search"
          clearable
          class="search-input"
        />
        
        <el-select v-model="typeFilter" placeholder="凭证类型" clearable class="type-filter">
          <el-option label="所有类型" value="" />
          <el-option label="收入证明" value="income" />
          <el-option label="信用评分" value="credit" />
          <el-option label="跨境信用" value="cross_border" />
        </el-select>
      </div>
      
      <el-table
        :data="filteredCredentials"
        style="width: 100%"
        v-loading="loading"
        empty-text="暂无金融凭证"
        :row-class-name="tableRowClassName"
      >
        <el-table-column label="类型" min-width="120">
          <template #default="scope">
            <el-tag
              :type="getCredentialTagType(scope.row.credential_type)"
            >
              {{ getCredentialTypeText(scope.row.credential_type) }}
            </el-tag>
          </template>
        </el-table-column>
        
        <el-table-column prop="personal_id" label="个人ID" min-width="120" />
        
        <el-table-column label="属性" min-width="180">
          <template #default="scope">
            <div v-if="scope.row.income_level">
              收入等级: {{ getIncomeLevelText(scope.row.income_level) }}
            </div>
            <div v-if="scope.row.credit_score_range">
              信用评分: {{ getCreditScoreText(scope.row.credit_score_range) }}
            </div>
          </template>
        </el-table-column>
        
        <el-table-column prop="issuer_id" label="发行机构" min-width="120" />
        
        <el-table-column label="有效期" min-width="200">
          <template #default="scope">
            <div>发行: {{ formatDate(scope.row.issue_date) }}</div>
            <div>过期: {{ formatDate(scope.row.expiry_date) }}</div>
            <el-tag 
              size="small"
              :type="isExpired(scope.row.expiry_date) ? 'danger' : 'success'"
            >
              {{ isExpired(scope.row.expiry_date) ? '已过期' : '有效' }}
            </el-tag>
          </template>
        </el-table-column>
        
        <el-table-column label="操作" fixed="right" width="200">
          <template #default="scope">
            <el-button 
              size="small" 
              @click="viewCredential(scope.row)"
              icon="el-icon-view"
            >
              查看
            </el-button>
            <el-button
              size="small"
              type="primary"
              @click="shareCredential(scope.row)"
              icon="el-icon-share"
              :disabled="isExpired(scope.row.expiry_date)"
            >
              分享
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
    
    <!-- 查看凭证详情对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="凭证详情"
      width="50%"
    >
      <div v-if="selectedCredential">
        <el-descriptions title="凭证信息" :column="1" border>
          <el-descriptions-item label="凭证类型">
            {{ getCredentialTypeText(selectedCredential.credential_type) }}
          </el-descriptions-item>
          <el-descriptions-item label="个人ID">
            {{ selectedCredential.personal_id }}
          </el-descriptions-item>
          <el-descriptions-item label="收入等级" v-if="selectedCredential.income_level">
            {{ getIncomeLevelText(selectedCredential.income_level) }}
          </el-descriptions-item>
          <el-descriptions-item label="信用评分" v-if="selectedCredential.credit_score_range">
            {{ getCreditScoreText(selectedCredential.credit_score_range) }}
          </el-descriptions-item>
          <el-descriptions-item label="发行机构">
            {{ selectedCredential.issuer_id }}
          </el-descriptions-item>
          <el-descriptions-item label="发行日期">
            {{ formatDate(selectedCredential.issue_date) }}
          </el-descriptions-item>
          <el-descriptions-item label="过期日期">
            {{ formatDate(selectedCredential.expiry_date) }}
          </el-descriptions-item>
          <el-descriptions-item label="凭证哈希">
            <el-tag size="small" type="info">{{ selectedCredential.hash }}</el-tag>
          </el-descriptions-item>
        </el-descriptions>
      </div>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="dialogVisible = false">关闭</el-button>
          <el-button type="primary" @click="copyHash" :disabled="!selectedCredential">复制哈希</el-button>
        </span>
      </template>
    </el-dialog>
    
    <!-- 分享凭证对话框 -->
    <el-dialog
      v-model="shareDialogVisible"
      title="分享凭证"
      width="50%"
    >
      <div v-if="selectedCredential">
        <p>您可以通过以下方式分享您的凭证：</p>
        
        <el-alert
          title="隐私保护提示"
          type="info"
          description="分享凭证仅会提供您授权的信息，不会泄露您的实际收入或确切信用分数。"
          :closable="false"
          show-icon
          style="margin-bottom: 20px;"
        />
        
        <div class="share-methods">
          <el-tabs v-model="shareMethod">
            <el-tab-pane label="分享哈希值" name="hash">
              <el-input
                v-model="selectedCredential.hash"
                placeholder="凭证哈希"
                readonly
              >
                <template #append>
                  <el-button @click="copyHash">复制</el-button>
                </template>
              </el-input>
              <p class="tip">您可以将此哈希值发送给需要验证您凭证的机构或个人。</p>
            </el-tab-pane>
            <el-tab-pane label="生成二维码" name="qrcode">
              <div class="qrcode-container">
                <div class="qrcode">
                  <!-- 实际项目中可以引入二维码生成组件 -->
                  <img src="https://via.placeholder.com/200x200?text=QR+Code" alt="QR Code" />
                </div>
              </div>
              <p class="tip">扫描二维码可以直接访问您的凭证验证页面。</p>
            </el-tab-pane>
          </el-tabs>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script lang="ts" setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { ElMessage } from 'element-plus'
import { financialApi } from '@/api/financial'
import type { FinancialCredential } from '@/types/financial'
import { useClipboard } from '@vueuse/core'
import { format } from 'date-fns'

const { copy } = useClipboard()

const loading = ref(false)
const credentials = ref<FinancialCredential[]>([])
const searchQuery = ref('')
const typeFilter = ref('')
const dialogVisible = ref(false)
const shareDialogVisible = ref(false)
const selectedCredential = ref<FinancialCredential | null>(null)
const shareMethod = ref('hash')

// 获取凭证列表
const fetchCredentials = async () => {
  loading.value = true
  try {
    const res = await financialApi.listFinancial()
    credentials.value = res.data?.data || []
  } catch (error: any) {
    ElMessage.error(error.message || '获取凭证列表失败')
  } finally {
    loading.value = false
  }
}

// 筛选凭证
const filteredCredentials = computed(() => {
  return credentials.value.filter(credential => {
    const matchesQuery = 
      !searchQuery.value || 
      credential.personal_id.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      credential.issuer_id.toLowerCase().includes(searchQuery.value.toLowerCase())
    
    const matchesType = !typeFilter.value || credential.credential_type === typeFilter.value
    
    return matchesQuery && matchesType
  })
})

// 格式化日期
const formatDate = (dateString: string) => {
  try {
    const date = new Date(dateString)
    return format(date, 'yyyy-MM-dd')
  } catch (e) {
    return dateString
  }
}

// 检查是否过期
const isExpired = (dateString: string) => {
  try {
    const expiry = new Date(dateString)
    return new Date() > expiry
  } catch (e) {
    return false
  }
}

// 表格行样式
const tableRowClassName = ({ row }: { row: FinancialCredential }) => {
  return isExpired(row.expiry_date) ? 'expired-row' : ''
}

// 凭证类型标签样式
const getCredentialTagType = (type: string) => {
  switch (type) {
    case 'income': return 'success'
    case 'credit': return 'warning'
    case 'cross_border': return 'primary'
    default: return 'info'
  }
}

// 凭证类型文本
const getCredentialTypeText = (type: string) => {
  switch (type) {
    case 'income': return '收入证明'
    case 'credit': return '信用评分'
    case 'cross_border': return '跨境信用'
    default: return type
  }
}

// 收入等级文本
const getIncomeLevelText = (level: string) => {
  switch (level) {
    case 'level_1': return '低收入 (低于$30,000)'
    case 'level_2': return '中低收入 ($30,000-$60,000)'
    case 'level_3': return '中等收入 ($60,000-$100,000)'
    case 'level_4': return '中高收入 ($100,000-$150,000)'
    case 'level_5': return '高收入 (高于$150,000)'
    default: return level
  }
}

// 信用评分文本
const getCreditScoreText = (range: string) => {
  switch (range) {
    case 'poor': return '较差 (300-579)'
    case 'fair': return '一般 (580-669)'
    case 'good': return '良好 (670-739)'
    case 'very_good': return '优秀 (740-799)'
    case 'excellent': return '极佳 (800-850)'
    default: return range
  }
}

// 查看凭证详情
const viewCredential = (credential: FinancialCredential) => {
  selectedCredential.value = credential
  dialogVisible.value = true
}

// 分享凭证
const shareCredential = (credential: FinancialCredential) => {
  selectedCredential.value = credential
  shareDialogVisible.value = true
}

// 复制哈希值
const copyHash = () => {
  if (selectedCredential.value?.hash) {
    copy(selectedCredential.value.hash)
    ElMessage.success('凭证哈希已复制到剪贴板')
  }
}

// 页面加载时获取凭证列表
onMounted(() => {
  fetchCredentials()
})
</script>

<style scoped>
.my-credentials-container {
  max-width: 1200px;
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

.credential-actions {
  display: flex;
  margin-bottom: 20px;
  gap: 10px;
}

.search-input {
  flex: 1;
}

.type-filter {
  width: 150px;
}

:deep(.expired-row) {
  color: #C0C4CC;
  background-color: #F5F7FA;
}

.qrcode-container {
  display: flex;
  justify-content: center;
  margin: 20px 0;
}

.qrcode {
  padding: 10px;
  border: 1px solid #EBEEF5;
  border-radius: 4px;
}

.tip {
  color: #606266;
  font-size: 14px;
  margin-top: 10px;
  text-align: center;
}

.share-methods {
  margin-top: 20px;
}
</style>
