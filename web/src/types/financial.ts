// 收入证明请求数据
export interface IncomeProofData {
  personal_id: string;
  actual_income: number;
  issuer_id: string;
  expiry_date: string;
}

// 信用评分请求数据
export interface CreditScoreData {
  personal_id: string;
  credit_score: number;
  issuer_id: string;
  expiry_date: string;
}

// 跨境信用请求数据
export interface CrossBorderData {
  personal_id: string;
  income_level: string;
  credit_score_range: string;
  issuer_id: string;
  expiry_date: string;
}

// 金融凭证验证请求数据
export interface FinancialVerifyData {
  hash: string;
}

// 金融凭证定义
export interface FinancialCredential {
  personal_id: string;
  income_level?: string;
  credit_score_range?: string;
  credential_type: string;
  issuer_id: string;
  issue_date: string;
  expiry_date: string;
  hash: string;
}

// 角色定义
export enum UserRole {
  Individual = 'individual',           // 个人用户
  FinancialInstitution = 'financial',  // 金融机构
  Verifier = 'verifier'                // 验证方（如银行、雇主等）
}

// 凭证类型
export enum CredentialType {
  Income = 'income',             // 收入证明
  CreditScore = 'credit',        // 信用评分
  CrossBorder = 'cross_border'   // 跨境信用证明
}
