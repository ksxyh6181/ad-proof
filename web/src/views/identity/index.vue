<template>
  <div class="identity-container">
    <el-card class="box-card">
      <template #header>
        <div class="card-header">
          <span>AI 数字身份管理系统</span>
        </div>
      </template>

      <el-tabs v-model="activeTab">
        <!-- Tab 1: Create Personhood -->
        <el-tab-pane label="创建人类数字身份 (Personhood)" name="create">
          <el-form :model="identityForm" label-width="120px" @submit.prevent>
            <el-form-item label="真实身份ID">
              <el-input v-model="identityForm.personal_id" placeholder="如护照号、身份证号等 (验证后将生成ZK证明并抛弃明文)" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" :loading="loading" @click="handleCreateIdentity">生成数字身份</el-button>
            </el-form-item>
          </el-form>

          <div v-if="personhoodHash" class="result-box">
            <h4>您的数字身份 Hash:</h4>
            <el-alert :title="personhoodHash" type="success" :closable="false" />
            <p class="desc">此 Hash 是您的链上唯一凭证（模拟），请妥善保存以便后续操作。</p>
          </div>
        </el-tab-pane>

        <!-- Tab 2: Register Agent -->
        <el-tab-pane label="注册 AI Agent" name="register">
          <el-form :model="agentForm" label-width="150px" @submit.prevent>
            <el-form-item label="人类数字身份 Hash">
              <el-input v-model="agentForm.owner_personhood_hash" placeholder="输入您的人类数字身份 Hash" />
            </el-form-item>
            <el-form-item label="AI Agent 公钥">
              <el-input v-model="agentForm.agent_pubkey" placeholder="AI Agent 的公钥（如 DID）" />
            </el-form-item>
            <el-form-item>
              <el-button type="primary" :loading="loading" @click="handleRegisterAgent">授权并绑定 Agent</el-button>
            </el-form-item>
          </el-form>

          <div v-if="lastAgentProof" class="result-box">
            <h4>AI Agent 绑定成功</h4>
            <p>ZKP Proof (Base64截断): {{ lastAgentProof.proof.substring(0, 50) }}...</p>
            <el-button type="info" size="small" @click="handleVerify(lastAgentProof)">立即验证该Agent</el-button>
          </div>
        </el-tab-pane>
      </el-tabs>
    </el-card>

    <br />

    <!-- 验证结果展示区 -->
    <el-card v-if="verifyResult" class="box-card">
      <template #header>
        <div class="card-header">
          <span>验证结果</span>
        </div>
      </template>
      <el-alert
        v-if="verifyResult.valid"
        :title="verifyResult.message"
        type="success"
        show-icon
        :closable="false"
      />
      <el-alert
        v-else
        :title="verifyResult.message"
        type="error"
        show-icon
        :closable="false"
      />
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { ElMessage } from 'element-plus'
import { createIdentity, registerAgent, verifyAgent } from '@/api/identity'

const activeTab = ref('create')
const loading = ref(false)

const identityForm = reactive({
  personal_id: ''
})

const agentForm = reactive({
  owner_personhood_hash: '',
  agent_pubkey: ''
})

const personhoodHash = ref('')
const lastAgentProof = ref<any>(null)
const verifyResult = ref<{ valid: boolean; message: string } | null>(null)

const handleCreateIdentity = async () => {
  if (!identityForm.personal_id) {
    ElMessage.warning('请输入您的真实身份ID')
    return
  }
  loading.value = true
  try {
    const res: any = await createIdentity({ personal_id: identityForm.personal_id })
    if (res.code === 200) {
      ElMessage.success('创建成功')
      personhoodHash.value = res.data.personhood_hash
      agentForm.owner_personhood_hash = personhoodHash.value
    } else {
      ElMessage.error(res.message || '创建失败')
    }
  } catch (error) {
    ElMessage.error('API请求失败')
  } finally {
    loading.value = false
  }
}

const handleRegisterAgent = async () => {
  if (!agentForm.owner_personhood_hash || !agentForm.agent_pubkey) {
    ElMessage.warning('请填写所有必填项')
    return
  }
  loading.value = true
  try {
    const res: any = await registerAgent({
      owner_personhood_hash: agentForm.owner_personhood_hash,
      agent_pubkey: agentForm.agent_pubkey
    })
    if (res.code === 200) {
      ElMessage.success('注册成功')
      lastAgentProof.value = res.data.proof
    } else {
      ElMessage.error(res.message || '注册失败')
    }
  } catch (error) {
    ElMessage.error('API请求失败')
  } finally {
    loading.value = false
  }
}

const handleVerify = async (proof: any) => {
  loading.value = true
  verifyResult.value = null
  try {
    const res: any = await verifyAgent({ proof })
    if (res.code === 200) {
      verifyResult.value = {
        valid: res.data.valid,
        message: res.data.message
      }
    } else {
      verifyResult.value = {
        valid: false,
        message: '验证请求失败'
      }
    }
  } catch (error) {
    ElMessage.error('API请求失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.identity-container {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.box-card {
  margin-bottom: 20px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: bold;
}

.result-box {
  margin-top: 20px;
  padding: 15px;
  background-color: #f5f7fa;
  border-radius: 4px;
}

.desc {
  font-size: 13px;
  color: #909399;
  margin-top: 10px;
}
</style>
