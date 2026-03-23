export type DemoRole = 'holder' | 'verifier' | 'financial_institution' | 'kyc_provider'

export const DEMO_ROLES: Array<{ label: string; value: DemoRole }> = [
  { label: '持证人 holder', value: 'holder' },
  { label: '验证方 verifier', value: 'verifier' },
  { label: '收入签发方 financial institution', value: 'financial_institution' },
  { label: 'KYC 签发方 kyc provider', value: 'kyc_provider' }
]

const ROLE_STORAGE_KEY = 'ad-proof-role'
const DEFAULT_ROLE: DemoRole = 'holder'

export const getStoredRole = (): DemoRole => {
  const saved = window.localStorage.getItem(ROLE_STORAGE_KEY)
  if (saved && DEMO_ROLES.some((role) => role.value === saved)) {
    return saved as DemoRole
  }

  window.localStorage.setItem(ROLE_STORAGE_KEY, DEFAULT_ROLE)
  return DEFAULT_ROLE
}

export const setStoredRole = (role: DemoRole) => {
  window.localStorage.setItem(ROLE_STORAGE_KEY, role)
  window.dispatchEvent(new CustomEvent('role-change', { detail: role }))
}
