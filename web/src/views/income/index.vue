<template>
  <div class="page-shell">
    <section class="hero-panel scenario-hero">
      <div class="scenario-copy">
        <span class="eyebrow">Scenario A · Income Threshold</span>
        <h1 class="section-title">证明“收入达到门槛”，而不是泄露精确收入。</h1>
        <p class="section-copy">
          这个页面把收入 VC 流程拆成三步：签发、生成 presentation、验证。
          证明语义对齐后端接口：`income tier >= required tier`，并在验证时同时检查 issuer、签名、有效期和状态。
        </p>
      </div>
      <div class="metric-grid">
        <div class="metric-box">
          <strong>推荐 issuer</strong>
          <p class="card-copy mono">bank_demo_001</p>
        </div>
        <div class="metric-box">
          <strong>等级映射</strong>
          <p class="card-copy">Tier 0: &lt;3000，Tier 1: ≥3000，Tier 2: ≥5000，Tier 3: ≥10000</p>
        </div>
      </div>
    </section>

    <section class="card-grid">
      <article class="content-card card-span-4 work-card">
        <div class="work-head">
          <span class="stage-badge">Step 1</span>
          <div>
            <p class="card-kicker">Issuer</p>
            <h2 class="card-heading">签发收入凭证头</h2>
          </div>
        </div>

        <el-form label-position="top" class="work-form">
          <el-form-item label="Subject Reference">
            <el-input v-model="issueForm.subject_ref" placeholder="例如 tenant-001" />
          </el-form-item>
          <el-form-item label="月收入金额">
            <el-input-number v-model="issueForm.actual_income" :min="0" :step="500" class="full-width" />
          </el-form-item>
          <el-form-item label="Issuer ID">
            <el-input v-model="issueForm.issuer_id" placeholder="bank_demo_001" />
          </el-form-item>
          <el-form-item label="过期日期">
            <el-date-picker
              v-model="issueForm.expires_at"
              class="full-width"
              type="date"
              value-format="YYYY-MM-DD"
              placeholder="选择日期"
            />
          </el-form-item>
        </el-form>

        <div class="action-row">
          <button class="accent-button" type="button" :disabled="issueLoading" @click="handleIssue">
            {{ issueLoading ? '签发中...' : '以 financial_institution 身份签发' }}
          </button>
          <button class="ghost-button" type="button" @click="fillIssuerExample">填充示例</button>
        </div>

        <div class="summary-list" v-if="issuedCredential">
          <div class="summary-row">
            <span class="summary-label">Credential ID</span>
            <span class="summary-value mono">{{ issuedCredential.credential_id }}</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">签发方</span>
            <span class="summary-value">{{ issuedCredential.issuer_id }}</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">有效期</span>
            <span class="summary-value">{{ issuedCredential.expires_at }}</span>
          </div>
        </div>

        <pre class="json-viewer">{{ issuedCredentialJson }}</pre>
      </article>

      <article class="content-card card-span-4 work-card">
        <div class="work-head">
          <span class="stage-badge secondary">Step 2</span>
          <div>
            <p class="card-kicker">Holder</p>
            <h2 class="card-heading">生成最小披露 presentation</h2>
          </div>
        </div>

        <el-form label-position="top" class="work-form">
          <el-form-item label="Credential ID">
            <el-input v-model="presentForm.credential_id" placeholder="先完成上一步签发" />
          </el-form-item>
          <el-form-item label="平台要求等级">
            <el-select v-model="presentForm.required_tier" class="full-width">
              <el-option
                v-for="option in tierOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
          </el-form-item>
        </el-form>

        <div class="action-row">
          <button class="accent-button" type="button" :disabled="presentLoading" @click="handlePresent">
            {{ presentLoading ? '生成中...' : '以 holder 身份生成证明' }}
          </button>
          <button class="ghost-button" type="button" @click="applyLatestCredential">使用最新凭证</button>
        </div>

        <div class="pill-row">
          <span class="pill">公开输入包含 credential_id 与 required_tier</span>
          <span class="pill">精确收入不会出现在 verifier 侧</span>
        </div>

        <pre class="json-viewer">{{ presentationJson }}</pre>
      </article>

      <article class="content-card card-span-4 work-card">
        <div class="work-head">
          <span class="stage-badge">Step 3</span>
          <div>
            <p class="card-kicker">Verifier</p>
            <h2 class="card-heading">校验 presentation</h2>
          </div>
        </div>

        <el-form label-position="top" class="work-form">
          <el-form-item label="Presentation JSON">
            <el-input
              v-model="verifyPayload"
              type="textarea"
              :rows="16"
              placeholder="可直接使用上一步输出，也可以粘贴 presentation JSON"
            />
          </el-form-item>
        </el-form>

        <div class="action-row">
          <button class="accent-button" type="button" :disabled="verifyLoading" @click="handleVerify">
            {{ verifyLoading ? '验证中...' : '以 verifier 身份验证' }}
          </button>
          <button class="ghost-button" type="button" @click="useLatestPresentation">使用最新 presentation</button>
        </div>

        <div class="summary-list" v-if="verification">
          <div class="summary-row">
            <span class="summary-label">验证结果</span>
            <span class="summary-value">{{ verification.valid ? '通过' : '失败' }}</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">业务语义</span>
            <span class="summary-value">{{ verification.message }}</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">过期日期</span>
            <span class="summary-value">{{ verification.expires_at }}</span>
          </div>
        </div>

        <pre class="json-viewer">{{ verificationJson }}</pre>
      </article>
    </section>

    <section class="card-grid">
      <article class="info-card card-span-6">
        <p class="card-kicker">Verifier 真正在查什么</p>
        <h3 class="card-heading">不是“这份哈希像不像”，而是四个业务问题</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label">1. 这份 credential 是不是可信 issuer 签的</span>
            <span class="summary-value">校验 header signature 与 issuer_id</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">2. 条件是否成立</span>
            <span class="summary-value">`income tier >= required tier`</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">3. 是否过期</span>
            <span class="summary-value">检查 `expires_at`</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">4. 状态是否正常</span>
            <span class="summary-value">当前 demo 中要求 `status = Active`</span>
          </div>
        </div>
      </article>

      <article class="info-card card-span-6">
        <p class="card-kicker">公开与私有</p>
        <h3 class="card-heading">收入页的数据边界</h3>
        <div class="metric-grid">
          <div class="metric-box">
            <strong>公开</strong>
            <p class="card-copy">凭证头、所需等级、公共输入、proof 字节串。</p>
          </div>
          <div class="metric-box">
            <strong>私有</strong>
            <p class="card-copy">真实月收入金额，仅由 issuer 与 holder 在签发/生成阶段接触。</p>
          </div>
        </div>
      </article>
    </section>
  </div>
</template>

<script lang="ts" setup>
import { computed, reactive, ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { createIncomePresentation, issueIncomeCredential, verifyIncomePresentation } from '@/api/vc'
import type { CredentialHeader, IncomePresentation, PresentationVerification } from '@/types/vc'
import { readStoredJson, writeStoredJson } from '@/utils/json-storage'
import { setStoredRole } from '@/utils/role'

const STORAGE_KEYS = {
  credential: 'income-credential-header',
  presentation: 'income-presentation',
  verification: 'income-verification'
}

const tierOptions = [
  { value: 0, label: 'Tier 0 · 基础访问（< 3000）' },
  { value: 1, label: 'Tier 1 · 月收入至少 3000' },
  { value: 2, label: 'Tier 2 · 月收入至少 5000' },
  { value: 3, label: 'Tier 3 · 月收入至少 10000' }
]

const nextYear = () => {
  const target = new Date()
  target.setFullYear(target.getFullYear() + 1)
  return target.toISOString().slice(0, 10)
}

const issueForm = reactive({
  subject_ref: 'tenant-demo-001',
  actual_income: 6800,
  issuer_id: 'bank_demo_001',
  expires_at: nextYear()
})

const presentForm = reactive({
  credential_id: '',
  required_tier: 2
})

const issuedCredential = ref<CredentialHeader | null>(readStoredJson<CredentialHeader>(STORAGE_KEYS.credential))
const presentation = ref<IncomePresentation | null>(readStoredJson<IncomePresentation>(STORAGE_KEYS.presentation))
const verification = ref<PresentationVerification | null>(
  readStoredJson<PresentationVerification>(STORAGE_KEYS.verification)
)
const verifyPayload = ref(presentation.value ? JSON.stringify(presentation.value, null, 2) : '')

const issueLoading = ref(false)
const presentLoading = ref(false)
const verifyLoading = ref(false)

if (issuedCredential.value) {
  presentForm.credential_id = issuedCredential.value.credential_id
}

watch(issuedCredential, (value) => {
  writeStoredJson(STORAGE_KEYS.credential, value)
})

watch(presentation, (value) => {
  writeStoredJson(STORAGE_KEYS.presentation, value)
  verifyPayload.value = value ? JSON.stringify(value, null, 2) : ''
})

watch(verification, (value) => {
  writeStoredJson(STORAGE_KEYS.verification, value)
})

const issuedCredentialJson = computed(() =>
  issuedCredential.value ? JSON.stringify(issuedCredential.value, null, 2) : '尚未签发收入凭证。'
)
const presentationJson = computed(() =>
  presentation.value ? JSON.stringify(presentation.value, null, 2) : '尚未生成 presentation。'
)
const verificationJson = computed(() =>
  verification.value ? JSON.stringify(verification.value, null, 2) : '尚未执行验证。'
)

const fillIssuerExample = () => {
  issueForm.subject_ref = 'tenant-demo-001'
  issueForm.actual_income = 6800
  issueForm.issuer_id = 'bank_demo_001'
  issueForm.expires_at = nextYear()
}

const applyLatestCredential = () => {
  if (!issuedCredential.value) {
    ElMessage.warning('还没有可用的 credential')
    return
  }
  presentForm.credential_id = issuedCredential.value.credential_id
}

const useLatestPresentation = () => {
  if (!presentation.value) {
    ElMessage.warning('还没有可用的 presentation')
    return
  }
  verifyPayload.value = JSON.stringify(presentation.value, null, 2)
}

const handleIssue = async () => {
  issueLoading.value = true
  setStoredRole('financial_institution')
  try {
    const result = await issueIncomeCredential({
      subject_ref: issueForm.subject_ref,
      actual_income: Number(issueForm.actual_income),
      issuer_id: issueForm.issuer_id,
      expires_at: issueForm.expires_at
    })
    issuedCredential.value = result
    presentForm.credential_id = result.credential_id
    presentation.value = null
    verification.value = null
  } finally {
    issueLoading.value = false
  }
}

const handlePresent = async () => {
  if (!presentForm.credential_id) {
    ElMessage.warning('请先提供 credential_id')
    return
  }

  presentLoading.value = true
  setStoredRole('holder')
  try {
    const result = await createIncomePresentation({
      credential_id: presentForm.credential_id,
      required_tier: presentForm.required_tier
    })
    presentation.value = result
    verification.value = null
  } finally {
    presentLoading.value = false
  }
}

const handleVerify = async () => {
  if (!verifyPayload.value.trim()) {
    ElMessage.warning('请先粘贴或生成 presentation JSON')
    return
  }

  let parsed: IncomePresentation
  try {
    parsed = JSON.parse(verifyPayload.value) as IncomePresentation
  } catch (_error) {
    ElMessage.error('Presentation JSON 解析失败')
    return
  }

  verifyLoading.value = true
  setStoredRole('verifier')
  try {
    verification.value = await verifyIncomePresentation(parsed)
  } finally {
    verifyLoading.value = false
  }
}
</script>

<style scoped>
.scenario-hero {
  display: grid;
  grid-template-columns: minmax(0, 1.1fr) minmax(280px, 0.9fr);
  gap: 20px;
}

.scenario-copy {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.work-card {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.work-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
}

.work-form {
  display: grid;
}

.action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.full-width {
  width: 100%;
}

@media (max-width: 980px) {
  .scenario-hero {
    grid-template-columns: 1fr;
  }
}
</style>
