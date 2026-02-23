import request from '@/utils/request'

export interface CreateIdentityRequest {
  personal_id: string
}

export interface RegisterAgentRequest {
  owner_personhood_hash: string
  agent_pubkey: string
}

export interface VerifyAgentRequest {
  proof: any // ZKProof
}

export function createIdentity(data: CreateIdentityRequest) {
  return request({
    url: '/identity/create',
    method: 'post',
    data,
    timeout: 30000 // ZK operations might take time
  })
}

export function registerAgent(data: RegisterAgentRequest) {
  return request({
    url: '/identity/agent/register',
    method: 'post',
    data,
    timeout: 30000
  })
}

export function verifyAgent(data: VerifyAgentRequest) {
  return request({
    url: '/identity/agent/verify',
    method: 'post',
    data,
    timeout: 30000
  })
}
