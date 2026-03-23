import axios from 'axios'
import { ElMessage } from 'element-plus'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse, InternalAxiosRequestConfig } from 'axios'
import { getStoredRole } from '@/utils/role'

interface ApiEnvelope<T> {
  code: number
  data: T | null
  msg: string
}

const apiBaseUrl =
  import.meta.env.VITE_API_BASE_URL || (import.meta.env.DEV ? 'http://127.0.0.1:8090/api' : '/api')

const service: AxiosInstance = axios.create({
  baseURL: apiBaseUrl,
  timeout: 10000
})

service.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    const role = getStoredRole()
    config.headers.set('X-Role', role)

    const method = config.method?.toLowerCase()
    if (method === 'post' || method === 'put' || method === 'patch') {
      config.headers.set('Content-Type', 'application/json;charset=UTF-8')
    }

    if (config.data !== undefined) {
      try {
        config.data = JSON.parse(JSON.stringify(config.data))
      } catch (_error) {
        ElMessage.error('请求数据序列化失败')
      }
    }

    return config
  },
  (error) => {
    ElMessage.error('请求发送失败')
    return Promise.reject(error)
  }
)

service.interceptors.response.use(
  <T>(response: AxiosResponse<ApiEnvelope<T>>) => {
    if (!response.data) {
      ElMessage.error('响应为空')
      return Promise.reject(new Error('响应为空'))
    }

    const { code, msg, data } = response.data
    if (code === 200) {
      if (msg && msg !== '访问成功') {
        ElMessage.success(msg)
      }
      return data as T
    }

    ElMessage.error(msg || '操作失败')
    return Promise.reject(new Error(msg || '操作失败'))
  },
  (error) => {
    if (error.response?.status === 403) {
      ElMessage.error('当前角色无权执行这个签发动作')
      return Promise.reject(error)
    }

    if (!error.response) {
      ElMessage.error('网络连接失败')
      return Promise.reject(error)
    }

    const message = error.response.data?.msg || '请求失败'
    ElMessage.error(message)
    return Promise.reject(error)
  }
)

const request = <T>(config: AxiosRequestConfig): Promise<T> => {
  return service(config) as Promise<T>
}

export default request
