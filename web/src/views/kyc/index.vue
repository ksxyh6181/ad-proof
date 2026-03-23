<template>
  <div class="page-shell">
    <section class="hero-panel scenario-hero">
      <div class="scenario-copy">
        <span class="eyebrow">Scenario B · KYC Level</span>
        <h1 class="section-title">证明“达到某个 KYC 等级”，不泄露完整身份档案。</h1>
        <p class="section-copy">
          当前后端把 KYC 语义收缩成等级条件证明。Verifier 只校验 `kyc level >= required level`，
          外加 issuer 签名、未过期与状态正常，不再索取不必要的身份字段。
        </p>
      </div>
      <div class="metric-grid">
        <div class="metric-box">
          <strong>推荐 issuer</strong>
          <p class="card-copy mono">kyc_demo_001</p>
        </div>
        <div class="metric-box">
          <strong>等级范围</strong>
          <p class="card-copy">Level 0 到 Level 3，可用于提现额度、交易准入与分层合规。</p>
        </div>
      </div>
    </section>

    <section class="card-grid">
      <article class="content-card card-span-4 work-card">
        <div class="work-head">
          <span class="stage-badge">Step 1</span>
          <div>
            <p class="card-kicker">Issuer</p>
            <h2 class="card-heading">签发 KYC 凭证头</h2>
          </div>
        </div>

        <el-form label-position="top" class="work-form">
          <el-form-item label="Subject Reference">
            <el-input v-model="issueForm.subject_ref" placeholder="例如 user-kyc-001" />
          </el-form-item>
          <el-form-item label="KYC Level">
            <el-select v-model="issueForm.kyc_level" class="full-width">
              <el-option
                v-for="option in levelOptions"
                :key="option.value"
                :label="option.label"
                :value="option.value"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="Issuer ID">
            <el-input v-model="issueForm.issuer_id" placeholder="kyc_demo_001" />
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
            {{ issueLoading ? '签发中...' : '以 kyc_provider 身份签发' }}
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
            <h2 class="card-heading">生成等级证明</h2>
          </div>
        </div>

        <el-form label-position="top" class="work-form">
          <el-form-item label="Credential ID">
            <el-input v-model="presentForm.credential_id" placeholder="先完成上一步签发" />
          </el-form-item>
          <el-form-item label="平台要求等级">
            <el-select v-model="presentForm.required_level" class="full-width">
              <el-option
                v-for="option in levelOptions"
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
          <span class="pill">公开输入仅包含所需等级</span>
          <span class="pill">完整身份档案不进入 verifier 侧</span>
        </div>

        <pre class="json-viewer">{{ presentationJson }}</pre>
      </article>

      <article class="content-card card-span-4 work-card">
        <div class="work-head">
          <span class="stage-badge">Step 3</span>
          <div>
            <p class="card-kicker">Verifier</p>
            <h2 class="card-heading">校验 KYC presentation</h2>
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
        <p class="card-kicker">业务语义</p>
        <h3 class="card-heading">为什么这个场景值得用 selective disclosure</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label">高风险交易平台</span>
            <span class="summary-value">只想知道用户是否达到规定 KYC 等级</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">提现额度升级</span>
            <span class="summary-value">需要合规门槛，不需要收走整套身份材料</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">隐私收益</span>
            <span class="summary-value">减少身份证明材料的重复暴露与二次滥用</span>
          </div>
        </div>
      </article>

      <article class="info-card card-span-6">
        <p class="card-kicker">Verifier 检查点</p>
        <h3 class="card-heading">页面展示与后端验证保持一致</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label">Issuer</span>
            <span class="summary-value">是否为可信 `kyc_demo_001`</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">Predicate</span>
            <span class="summary-value">`kyc level >= required level`</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">Validity</span>
            <span class="summary-value">未过期且状态为 Active</span>
          </div>
        </div>
      </article>
    </section>
  </div>
</template>

<script lang="ts" setup>
import { computed, reactive, ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { createKycPresentation, issueKycCredential, verifyKycPresentation } from '@/api/vc'
import type { CredentialHeader, KycPresentation, PresentationVerification } from '@/types/vc'
import { readStoredJson, writeStoredJson } from '@/utils/json-storage'
import { setStoredRole } from '@/utils/role'

const STORAGE_KEYS = {
  credential: 'kyc-credential-header',
  presentation: 'kyc-presentation',
  verification: 'kyc-verification'
}

const levelOptions = [
  { value: 0, label: 'Level 0 · 仅基础档案' },
  { value: 1, label: 'Level 1 · 已完成基础核验' },
  { value: 2, label: 'Level 2 · 已完成强化核验' },
  { value: 3, label: 'Level 3 · 已完成高强度核验' }
]

const nextYear = () => {
  const target = new Date()
  target.setFullYear(target.getFullYear() + 1)
  return target.toISOString().slice(0, 10)
}

const issueForm = reactive({
  subject_ref: 'user-kyc-001',
  kyc_level: 2,
  issuer_id: 'kyc_demo_001',
  expires_at: nextYear()
})

const presentForm = reactive({
  credential_id: '',
  required_level: 1
})

const issuedCredential = ref<CredentialHeader | null>(readStoredJson<CredentialHeader>(STORAGE_KEYS.credential))
const presentation = ref<KycPresentation | null>(readStoredJson<KycPresentation>(STORAGE_KEYS.presentation))
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
  issuedCredential.value ? JSON.stringify(issuedCredential.value, null, 2) : '尚未签发 KYC 凭证。'
)
const presentationJson = computed(() =>
  presentation.value ? JSON.stringify(presentation.value, null, 2) : '尚未生成 presentation。'
)
const verificationJson = computed(() =>
  verification.value ? JSON.stringify(verification.value, null, 2) : '尚未执行验证。'
)

const fillIssuerExample = () => {
  issueForm.subject_ref = 'user-kyc-001'
  issueForm.kyc_level = 2
  issueForm.issuer_id = 'kyc_demo_001'
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
  setStoredRole('kyc_provider')
  try {
    const result = await issueKycCredential({
      subject_ref: issueForm.subject_ref,
      kyc_level: issueForm.kyc_level,
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
    const result = await createKycPresentation({
      credential_id: presentForm.credential_id,
      required_level: presentForm.required_level
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

  let parsed: KycPresentation
  try {
    parsed = JSON.parse(verifyPayload.value) as KycPresentation
  } catch (_error) {
    ElMessage.error('Presentation JSON 解析失败')
    return
  }

  verifyLoading.value = true
  setStoredRole('verifier')
  try {
    verification.value = await verifyKycPresentation(parsed)
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
