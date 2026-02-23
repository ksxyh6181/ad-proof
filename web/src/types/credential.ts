// 凭证类型定义

// 旧版API类型 - 保持兼容性
export interface CredentialData {
  student_id: string;
  name: string;
  degree: string;
  graduation_date: string;
}

export interface VerifyData {
  hash: string;
}

/**
 * 凭证对象
 */
export interface Credential {
  id?: string;
  hash: string;
  type?: string;
  issuer?: string;
  issueDate?: string;
  metadataUri?: string;
  revoked?: boolean;
  onChain?: boolean;
  signature?: string;
  student_id?: string;
  name?: string;
  degree?: string;
  graduation_date?: string;
  public_inputs?: any;
  proof?: any;
}

/**
 * 凭证签发参数
 */
export interface CredentialIssueParams {
  issuer: string;
  type: string;
  content: string;
  metadataUri?: string;
  onChain?: boolean;
}

/**
 * 凭证验证参数
 */
export interface CredentialVerifyParams {
  hash: string;
  content?: string;
  checkOnChain?: boolean;
}

/**
 * Solana凭证注册/验证结果
 */
export interface SolanaCredentialResult {
  success: boolean;
  message?: string;
  signature?: string;
  error?: any;
  data?: any;
}

/**
 * Solana凭证对象
 */
export interface SolanaCredential {
  hash: string;
  credentialType: string;
  issuer: string;
  issueDate: number;
  metadataUri: string;
  revoked: boolean;
  authority: string;
}

// 调试日志条目类型
export interface DebugLogEntry {
  type: string;
  message: string;
  data?: any;
  timestamp: number;
}
