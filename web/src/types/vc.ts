export type CredentialKind = 'Income' | 'Kyc'
export type CredentialStatus = 'Active' | 'Revoked'

export interface CredentialHeader {
  credential_id: string
  kind: CredentialKind
  issuer_id: string
  subject_ref: string
  issued_at: string
  expires_at: string
  status: CredentialStatus
  signature: string
}

export interface ZkProof {
  proof: number[]
  public_inputs: string[]
}

export interface IncomePresentation {
  header: CredentialHeader
  required_tier: number
  proof: ZkProof
}

export interface KycPresentation {
  header: CredentialHeader
  required_level: number
  proof: ZkProof
}

export interface PresentationVerification {
  credential_id: string
  issuer_id: string
  kind: CredentialKind
  valid: boolean
  message: string
  expires_at: string
}

export interface IssueIncomeCredentialRequest {
  subject_ref: string
  actual_income: number
  issuer_id: string
  expires_at: string
}

export interface IssueKycCredentialRequest {
  subject_ref: string
  kyc_level: number
  issuer_id: string
  expires_at: string
}
