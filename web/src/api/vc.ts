import request from '@/utils/request'
import type {
  CredentialHeader,
  IncomePresentation,
  IssueIncomeCredentialRequest,
  IssueKycCredentialRequest,
  KycPresentation,
  PresentationVerification
} from '@/types/vc'

export const issueIncomeCredential = (data: IssueIncomeCredentialRequest) =>
  request<CredentialHeader>({
    url: '/vc/income/issue',
    method: 'post',
    data
  })

export const createIncomePresentation = (data: { credential_id: string; required_tier: number }) =>
  request<IncomePresentation>({
    url: '/vc/income/present',
    method: 'post',
    data
  })

export const verifyIncomePresentation = (presentation: IncomePresentation) =>
  request<PresentationVerification>({
    url: '/vc/income/verify',
    method: 'post',
    data: { presentation }
  })

export const issueKycCredential = (data: IssueKycCredentialRequest) =>
  request<CredentialHeader>({
    url: '/vc/kyc/issue',
    method: 'post',
    data
  })

export const createKycPresentation = (data: { credential_id: string; required_level: number }) =>
  request<KycPresentation>({
    url: '/vc/kyc/present',
    method: 'post',
    data
  })

export const verifyKycPresentation = (presentation: KycPresentation) =>
  request<PresentationVerification>({
    url: '/vc/kyc/verify',
    method: 'post',
    data: { presentation }
  })
