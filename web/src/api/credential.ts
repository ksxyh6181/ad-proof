import axios from 'axios';
import { SolanaCredentialClient } from '../utils/solana-client';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import type { Credential, CredentialIssueParams, CredentialVerifyParams, SolanaCredentialResult } from '../types/credential';


// 调试对象，用于记录操作和错误
export const SOLANA_NETWORK_URL = "http://127.0.0.1:8899";
export const debug = {
  logs: [] as { type: string; message: string; data?: any; timestamp: number }[],

  // 添加日志
  log(type: string, message: string, data?: any) {
    const logEntry = {
      type,
      message,
      data,
      timestamp: Date.now()
    };

    this.logs.unshift(logEntry);

    // 限制日志数量
    if (this.logs.length > 100) {
      this.logs.pop();
    }

    // 根据类型输出不同颜色的日志
    const styles = {
      API: 'color: blue',
      SOLANA: 'color: green',
      ERROR: 'color: red',
      UI: 'color: purple',
      OTHER: 'color: gray'
    };

    console.log(
      `%c[${type}] ${message}`,
      styles[type] || styles.OTHER,
      data || ''
    );

    return logEntry;
  },

  // 清除日志
  clear() {
    this.logs = [];
    console.clear();
    return this.logs;
  }
};

// API基础URL
const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:3001';
const CREDENTIAL_PATH = '/api/credentials';

/**
 * 创建一个新的凭证
 * @param params 凭证参数
 * @returns 创建的凭证
 */
export async function issueCredential(params: CredentialIssueParams): Promise<Credential> {
  debug.log('API', '开始签发凭证', params);

  try {
    const response = await axios.post(`${API_URL}${CREDENTIAL_PATH}/issue`, params);
    const credential = response.data;

    debug.log('API', '凭证签发成功', credential);

    // 如果需要上链，尝试注册凭证
    if (params.onChain) {
      try {
        debug.log('API', '准备将凭证上链', {
          hash: credential.hash,
          type: credential.type,
          issuer: credential.issuer
        });

        // 注册凭证到Solana
        const result = await registerCredentialOnChain(credential);

        debug.log('API', '凭证上链结果', result);
      } catch (error) {
        debug.log('ERROR', '凭证上链失败', {
          error: error.message,
          hash: credential.hash
        });
      }
    }

    return credential;
  } catch (error) {
    debug.log('ERROR', '凭证签发API请求失败', {
      error: error.message,
      params
    });
    throw error;
  }
}

/**
 * 验证凭证
 * @param params 验证参数
 * @returns 验证结果
 */
export async function verifyCredential(params: CredentialVerifyParams): Promise<{ valid: boolean; credential?: Credential }> {
  debug.log('API', '开始验证凭证', params);

  try {
    const response = await axios.post(`${API_URL}${CREDENTIAL_PATH}/verify`, params);
    const result = response.data;

    debug.log('API', '凭证验证结果', result);

    // 如果需要检查链上状态
    if (params.checkOnChain && result.valid && result.credential) {
      try {
        debug.log('API', '准备验证凭证链上状态', {
          hash: result.credential.hash
        });

        // 检查凭证在Solana上的状态
        const onChainValid = await verifyCredentialOnChain(result.credential.hash);

        // 更新验证结果，同时考虑链上状态
        result.valid = result.valid && onChainValid;

        debug.log('API', '链上验证结果', {
          hash: result.credential.hash,
          valid: onChainValid,
          finalResult: result.valid
        });
      } catch (error) {
        debug.log('ERROR', '链上验证失败', {
          error: error.message,
          hash: result.credential?.hash
        });
      }
    }

    return result;
  } catch (error) {
    debug.log('ERROR', '凭证验证API请求失败', {
      error: error.message,
      params
    });
    throw error;
  }
}

/**
 * 获取所有凭证
 * @returns 凭证列表
 */
export async function getAllCredentials(): Promise<Credential[]> {
  debug.log('API', '开始获取所有凭证');

  try {
    const response = await axios.get(`${API_URL}${CREDENTIAL_PATH}`);
    const credentials = response.data;

    debug.log('API', '获取所有凭证成功', {
      count: credentials.length
    });

    return credentials;
  } catch (error: any) {
    debug.log('ERROR', '获取所有凭证API请求失败', {
      error: error.message
    });
    throw error;
  }
}

/**
 * 获取单个凭证
 */
export async function getCredential(hash: string): Promise<Credential | null> {
  debug.log('API', '开始获取单个凭证', { hash });

  try {
    const response = await axios.post(`${API_URL}/api/credential/get`, { hash });
    const credential = response.data?.data || response.data;

    debug.log('API', '获取单个凭证成功', credential);
    return credential;
  } catch (error: any) {
    debug.log('ERROR', '获取单个凭证失败', { error: error.message });
    throw error;
  }
}
/**
 * 注册凭证到Solana区块链
 * @param credential 要注册的凭证
 * @returns 交易结果
 */
export async function registerCredentialOnChain(credential: Credential): Promise<SolanaCredentialResult> {
  debug.log('API', '开始将凭证注册到区块链', {
    hash: credential.hash,
    type: credential.type
  });

  try {
    // 1. 创建Solana连接
    const connection = new Connection(SOLANA_NETWORK_URL);

    // 2. 生成一个临时钱包用于测试 - 实际环境中应连接到用户的钱包
    const wallet = Keypair.generate();
    debug.log('SOLANA', '使用临时钱包', {
      publicKey: wallet.publicKey.toString()
    });

    // 3. 创建凭证客户端
    const client = new SolanaCredentialClient(connection, {
      publicKey: wallet.publicKey,
      signTransaction: async (tx) => {
        tx.partialSign(wallet);
        return tx;
      },
      signAllTransactions: async (txs) => {
        return txs.map(tx => {
          tx.partialSign(wallet);
          return tx;
        });
      }
    });

    // 4. 请求空投SOL以支付交易费用 (仅限开发环境)
    if (SOLANA_NETWORK_URL.includes('devnet') || SOLANA_NETWORK_URL.includes('localhost')) {
      debug.log('SOLANA', '请求SOL空投');

      try {
        const signature = await connection.requestAirdrop(
          wallet.publicKey,
          10_000_000 // 0.01 SOL (lamports)
        );

        await connection.confirmTransaction(signature);
        debug.log('SOLANA', 'SOL空投成功', { signature });
      } catch (error) {
        debug.log('ERROR', 'SOL空投失败', { error: error.message });
        // 空投失败不应阻止后续操作，因为钱包可能已经有足够的SOL
      }
    }

    // 5. 注册凭证
    const issueTimestamp = Math.floor(new Date(credential.issueDate).getTime() / 1000);

    debug.log('SOLANA', '准备注册凭证', {
      hash: credential.hash,
      type: credential.type,
      issuer: credential.issuer,
      issueTimestamp,
      metadataUri: credential.metadataUri || ""
    });

    const signature = await client.registerCredential(
      credential.hash,
      credential.type,
      credential.issuer,
      issueTimestamp,
      credential.metadataUri || ""
    );

    debug.log('SOLANA', '凭证注册成功', {
      signature,
      hash: credential.hash
    });

    return {
      success: true,
      signature,
      message: "凭证已成功注册到区块链"
    };
  } catch (error) {
    debug.log('ERROR', '凭证注册到区块链失败', {
      error: error.message,
      logs: error.logs,
      hash: credential.hash
    });

    return {
      success: false,
      message: `凭证注册失败: ${error.message}`,
      error: error
    };
  }
}

/**
 * 在Solana区块链上验证凭证
 * @param hash 凭证哈希
 * @returns 如果凭证在链上且有效则返回true，否则返回false
 */
export async function verifyCredentialOnChain(hash: string): Promise<boolean> {
  debug.log('API', '开始在区块链上验证凭证', { hash });

  try {
    // 1. 创建Solana连接
    const connection = new Connection(SOLANA_NETWORK_URL);

    // 2. 创建一个临时钱包 - 验证不需要签名交易，但API要求钱包
    const wallet = Keypair.generate();

    // 3. 创建凭证客户端
    const client = new SolanaCredentialClient(connection, {
      publicKey: wallet.publicKey,
      signTransaction: async (tx) => tx,
      signAllTransactions: async (txs) => txs
    });

    // 4. 验证凭证
    const isValid = await client.verifyCredential(hash);

    debug.log('SOLANA', '区块链凭证验证结果', {
      hash,
      isValid
    });

    return isValid;
  } catch (error) {
    debug.log('ERROR', '区块链凭证验证失败', {
      error: error.message,
      hash
    });

    // 出现错误时，视为验证失败
    return false;
  }
}

/**
 * 从Solana区块链获取凭证信息
 * @param hash 凭证哈希
 * @returns 凭证信息
 */
export async function getCredentialFromChain(hash: string): Promise<any | null> {
  debug.log('API', '开始从区块链获取凭证信息', { hash });

  try {
    // 1. 创建Solana连接
    const connection = new Connection(SOLANA_NETWORK_URL);

    // 2. 生成一个临时钱包 - 读取操作不需要签名能力
    const wallet = Keypair.generate();

    // 3. 创建凭证客户端
    const client = new SolanaCredentialClient(connection, {
      publicKey: wallet.publicKey,
      signTransaction: async (tx) => tx,
      signAllTransactions: async (txs) => txs
    });

    // 4. 获取凭证
    const credentialInfo = await client.getCredential(hash);

    if (credentialInfo) {
      debug.log('SOLANA', '成功从区块链获取凭证信息', credentialInfo);
    } else {
      debug.log('SOLANA', '区块链上未找到凭证', { hash });
    }

    return credentialInfo;
  } catch (error) {
    debug.log('ERROR', '从区块链获取凭证信息失败', {
      error: error.message,
      hash
    });

    return null;
  }
}

/**
 * 检查凭证哈希是否在链上
 * @param hash 凭证哈希
 * @returns 检查结果
 */
export async function checkCredentialOnChain(hash: string): Promise<SolanaCredentialResult> {
  debug.log('API', '开始检查凭证是否在链上', { hash });

  try {
    // 获取凭证信息
    const credentialInfo = await getCredentialFromChain(hash);

    if (credentialInfo) {
      return {
        success: true,
        message: "凭证在区块链上",
        data: credentialInfo
      };
    } else {
      return {
        success: false,
        message: "凭证不在区块链上",
        data: null
      };
    }
  } catch (error) {
    debug.log('ERROR', '检查凭证是否在链上失败', {
      error: error.message,
      hash
    });

    return {
      success: false,
      message: `检查失败: ${error.message}`,
      error: error
    };
  }
}
