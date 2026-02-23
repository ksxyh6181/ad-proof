<template>
  <div class="solana-credential-manager">
    <el-card>
      <template #header>
        <div class="card-header">
          <h3>Solana 凭证管理</h3>
        </div>
      </template>
      
      <el-alert
        v-if="error"
        :title="error"
        type="error"
        show-icon
        :closable="true"
        @close="error = ''"
      />
      
      <el-alert
        v-if="success"
        type="success"
        :title="success"
        show-icon
        :closable="true"
        @close="success = ''"
      />

      <el-tabs v-model="activeTab">
        <!-- 注册凭证选项卡 -->
        <el-tab-pane label="注册凭证" name="register">
          <el-form :model="registerForm" label-width="120px">
            <el-form-item label="凭证哈希">
              <el-input v-model="registerForm.hash" placeholder="输入凭证哈希值" />
              <div class="input-actions">
                <el-button size="small" @click="generateTestHash">生成测试哈希</el-button>
                <el-tooltip content="复制测试值">
                  <el-button size="small" @click="fillTestValues">填充测试数据</el-button>
                </el-tooltip>
              </div>
            </el-form-item>
            
            <el-form-item label="凭证类型">
              <el-input v-model="registerForm.credentialType" placeholder="如: 教育, 金融, 身份等" />
            </el-form-item>
            
            <el-form-item label="颁发者">
              <el-input v-model="registerForm.issuer" placeholder="颁发机构名称" />
            </el-form-item>
            
            <el-form-item label="元数据URI">
              <el-input v-model="registerForm.metadataUri" placeholder="如: ipfs://<hash>" />
            </el-form-item>
            
            <el-form-item>
              <el-button type="primary" @click="registerCredential" :loading="loading">
                注册到区块链
              </el-button>
              
              <!-- 添加检查哈希注册状态按钮 -->
              <el-button type="info" @click="checkHashOnChain" :disabled="!registerForm.hash">
                检查哈希注册状态
              </el-button>
            </el-form-item>
          </el-form>
          
          <!-- 添加调试信息显示 -->
          <div v-if="debugInfo" class="debug-info">
            <h4>调试信息:</h4>
            <el-alert
              v-if="debugInfo.onChain"
              type="success"
              title="哈希已上链!"
              description="此凭证哈希已在区块链上注册"
              show-icon
            />
            <el-alert
              v-else-if="debugInfo.onChain === false"
              type="warning"
              title="哈希未上链"
              description="此凭证哈希尚未在区块链上注册"
              show-icon
            />
            <div v-if="debugInfo.credential" class="credential-summary">
              <pre>{{ JSON.stringify(debugInfo.credential, null, 2) }}</pre>
            </div>
          </div>
        </el-tab-pane>
        
        <!-- 验证凭证选项卡 -->
        <el-tab-pane label="验证凭证" name="verify">
          <el-form :model="verifyForm" label-width="120px">
            <el-form-item label="凭证哈希">
              <el-input v-model="verifyForm.hash" placeholder="输入凭证哈希值" />
            </el-form-item>
            
            <el-form-item>
              <el-button type="primary" @click="verifyCredential" :loading="loading">
                验证凭证
              </el-button>
            </el-form-item>
          </el-form>
          
          <div v-if="verificationResult !== null" class="result-box">
            <h4>验证结果:</h4>
            <p :class="verificationResult ? 'valid' : 'invalid'">
              {{ verificationResult ? '✅ 凭证有效' : '❌ 凭证无效或已撤销' }}
            </p>
          </div>
        </el-tab-pane>
        
        <!-- 查看凭证选项卡 -->
        <el-tab-pane label="查看凭证" name="view">
          <el-form :model="viewForm" label-width="120px">
            <el-form-item label="凭证哈希">
              <el-input v-model="viewForm.hash" placeholder="输入凭证哈希值" />
            </el-form-item>
            
            <el-form-item>
              <el-button type="primary" @click="getCredential" :loading="loading">
                获取凭证信息
              </el-button>
            </el-form-item>
          </el-form>
          
          <div v-if="credentialInfo" class="credential-details">
            <h4>凭证详情:</h4>
            <el-descriptions :column="1" border>
              <el-descriptions-item label="哈希">{{ credentialInfo.hash }}</el-descriptions-item>
              <el-descriptions-item label="类型">{{ credentialInfo.credentialType }}</el-descriptions-item>
              <el-descriptions-item label="颁发者">{{ credentialInfo.issuer }}</el-descriptions-item>
              <el-descriptions-item label="颁发日期">
                {{ formatDate(credentialInfo.issueDate) }}
              </el-descriptions-item>
              <el-descriptions-item label="状态">
                {{ credentialInfo.revoked ? '已撤销' : '有效' }}
              </el-descriptions-item>
              <el-descriptions-item label="元数据URI">
                <a :href="formatIpfsUrl(credentialInfo.metadataUri)" target="_blank">
                  {{ credentialInfo.metadataUri }}
                </a>
              </el-descriptions-item>
              <el-descriptions-item label="所有者">
                {{ credentialInfo.owner }}
              </el-descriptions-item>
            </el-descriptions>
          </div>
        </el-tab-pane>
      </el-tabs>
      
      <!-- 交易结果显示 -->
      <div v-if="transaction" class="transaction-result">
        <h4>交易信息:</h4>
        <p>签名: 
          <a :href="getExplorerUrl(transaction)" target="_blank">
            {{ transaction.substring(0, 16) }}...
          </a>
        </p>
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { debug, issueCredential, verifyCredential as verifyCredentialApi, getCredentialFromChain, checkCredentialOnChain } from '@/api/credential';
import { format } from 'date-fns';

// 状态
const activeTab = ref('register');
const loading = ref(false);
const error = ref('');
const success = ref('');
const transaction = ref('');
const verificationResult = ref<boolean | null>(null);
const credentialInfo = ref<any>(null);
const debugInfo = ref<{
  onChain: boolean | null;
  credential?: any;
  error?: string;
  timestamp?: string;
} | null>(null);

// 表单数据
const registerForm = reactive({
  hash: '',
  credentialType: '',
  issuer: '',
  metadataUri: ''
});

const verifyForm = reactive({
  hash: ''
});

const viewForm = reactive({
  hash: ''
});

// 注册凭证到Solana区块链
async function registerCredential() {
  if (!registerForm.hash || !registerForm.credentialType || !registerForm.issuer) {
    error.value = '请填写所有必填字段';
    return;
  }
  
  loading.value = true;
  error.value = '';
  success.value = '';
  transaction.value = '';
  
  debug.log('UI', `开始注册凭证: ${registerForm.hash}`, registerForm);
  
  try {
    const result = await issueCredential({
      issuer: registerForm.issuer,
      type: registerForm.credentialType,
      content: registerForm.hash, // 使用哈希作为内容
      metadataUri: registerForm.metadataUri,
      onChain: true
    });
    
    if (result && result.hash) {
      success.value = `凭证注册成功: ${result.hash}`;
      transaction.value = result.signature || '';
      debug.log('UI', '凭证注册成功', result);
    } else {
      error.value = '凭证注册失败，没有返回有效结果';
      debug.log('ERROR', '凭证注册失败，无效结果', result);
    }
  } catch (err: any) {
    error.value = `凭证注册失败: ${err.message}`;
    debug.log('ERROR', '凭证注册异常', err);
  } finally {
    loading.value = false;
  }
}

// 生成测试哈希 - 用于调试
function generateTestHash() {
  const timestamp = Date.now();
  const randomPart = Math.random().toString(36).substring(2, 15);
  registerForm.hash = `test_${timestamp}_${randomPart}`;
  debug.log('UI', '生成测试哈希', { hash: registerForm.hash });
}

// 填充测试值
function fillTestValues() {
  generateTestHash();
  registerForm.credentialType = '学历证书';
  registerForm.issuer = '测试大学';
  registerForm.metadataUri = 'ipfs://QmTest';
  debug.log('UI', '填充测试数据', registerForm);
}

// 检查哈希是否已上链
async function checkHashOnChain() {
  if (!registerForm.hash) {
    error.value = '请输入凭证哈希';
    return;
  }
  
  loading.value = true;
  error.value = '';
  success.value = '';
  debugInfo.value = null;
  
  try {
    debug.log('UI', `检查哈希是否上链: ${registerForm.hash}`);
    
    // 检查凭证是否在链上
    const result = await checkCredentialOnChain(registerForm.hash);
    
    if (result.success && result.data) {
      debugInfo.value = {
        onChain: true,
        credential: result.data,
        timestamp: format(new Date(), 'yyyy-MM-dd HH:mm:ss')
      };
      success.value = `哈希已上链: ${registerForm.hash}`;
      debug.log('UI', `哈希已上链: ${registerForm.hash}`, result.data);
    } else {
      debugInfo.value = {
        onChain: false,
        error: result.message || '凭证未找到',
        timestamp: format(new Date(), 'yyyy-MM-dd HH:mm:ss')
      };
      error.value = `哈希未上链: ${registerForm.hash}`;
      debug.log('UI', `哈希未上链: ${registerForm.hash}`, result.message);
    }
  } catch (err: any) {
    debugInfo.value = {
      onChain: null,
      error: err.message,
      timestamp: format(new Date(), 'yyyy-MM-dd HH:mm:ss')
    };
    error.value = `检查哈希失败: ${err.message}`;
    debug.log('ERROR', '检查哈希异常', err);
  } finally {
    loading.value = false;
  }
}

// 验证凭证
async function verifyCredential() {
  if (!verifyForm.hash) {
    error.value = '请输入凭证哈希';
    return;
  }
  
  loading.value = true;
  error.value = '';
  success.value = '';
  verificationResult.value = null;
  
  try {
    const result = await verifyCredentialApi({
      hash: verifyForm.hash,
      checkOnChain: true
    });
    
    if (result.valid) {
      verificationResult.value = true;
      success.value = `凭证验证成功: ${verifyForm.hash}`;
      debug.log('UI', '凭证验证成功', result);
    } else {
      verificationResult.value = false;
      error.value = `凭证验证失败: ${verifyForm.hash}`;
      debug.log('UI', '凭证验证失败', result);
    }
  } catch (err: any) {
    error.value = `验证凭证失败: ${err.message}`;
    debug.log('ERROR', '验证凭证异常', err);
  } finally {
    loading.value = false;
  }
}

// 获取凭证信息
async function getCredential() {
  if (!viewForm.hash) {
    error.value = '请输入凭证哈希';
    return;
  }
  
  loading.value = true;
  error.value = '';
  success.value = '';
  credentialInfo.value = null;
  
  try {
    const result = await getCredentialFromChain(viewForm.hash);
    
    if (result) {
      credentialInfo.value = result;
      success.value = `获取凭证成功: ${viewForm.hash}`;
      debug.log('UI', '获取凭证成功', result);
    } else {
      error.value = `未找到凭证: ${viewForm.hash}`;
      debug.log('UI', '未找到凭证', { hash: viewForm.hash });
    }
  } catch (err: any) {
    error.value = `获取凭证失败: ${err.message}`;
    debug.log('ERROR', '获取凭证异常', err);
  } finally {
    loading.value = false;
  }
}

// 格式化日期
function formatDate(timestamp: any) {
  if (!timestamp) return 'N/A';
  
  // Anchor BN对象需要转换
  let dateValue;
  if (typeof timestamp.toNumber === 'function') {
    dateValue = new Date(timestamp.toNumber() * 1000);
  } else {
    dateValue = new Date(Number(timestamp) * 1000);
  }
  
  return format(dateValue, 'yyyy-MM-dd HH:mm:ss');
}

// 格式化IPFS URL为可访问的网关URL
function formatIpfsUrl(uri: any) {
  if (!uri) return '#';
  if (uri.startsWith('ipfs://')) {
    return uri.replace('ipfs://', 'https://ipfs.io/ipfs/');
  }
  return uri;
}

// 获取交易浏览器URL
function getExplorerUrl(signature: any) {
  // 默认使用devnet
  return `https://explorer.solana.com/tx/${signature}?cluster=devnet`;
}
</script>

<style scoped>
.solana-credential-manager {
  max-width: 800px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.result-box {
  margin-top: 20px;
  padding: 15px;
  border-radius: 4px;
  background-color: #f8f9fa;
}

.valid {
  color: #67c23a;
  font-weight: bold;
}

.invalid {
  color: #f56c6c;
  font-weight: bold;
}

.credential-details {
  margin-top: 20px;
}

.transaction-result {
  margin-top: 20px;
  padding: 15px;
  border-radius: 4px;
  background-color: #f0f9eb;
}

.debug-info {
  margin-top: 20px;
  padding: 15px;
  border-radius: 4px;
  background-color: #f8f9fa;
  border: 1px solid #e9ecef;
}

.credential-summary {
  margin-top: 10px;
  padding: 10px;
  background-color: #f0f0f0;
  border-radius: 4px;
  overflow-x: auto;
}

.credential-summary pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}

.input-actions {
  display: flex;
  gap: 8px;
  margin-top: 8px;
}
</style>
