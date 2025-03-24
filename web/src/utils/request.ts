import axios from 'axios'
import { ElMessage } from 'element-plus'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios'

const service: AxiosInstance = axios.create({
  baseURL: '/api',
  timeout: 10000
})

// 请求拦截器
service.interceptors.request.use(
  (config) => {
    const role = localStorage.getItem('role')
    if (!role) {
      ElMessage.error('请先选择角色')
      return Promise.reject(new Error('未选择角色'))
    }
    config.headers['X-Role'] = role
    
    // 确保Content-Type正确设置为application/json
    if (config.method?.toLowerCase() === 'post' || config.method?.toLowerCase() === 'put') {
      config.headers['Content-Type'] = 'application/json;charset=UTF-8'
    }
    
    // 深度克隆数据以避免修改原始引用
    if (config.data) {
      try {
        // 确保数据是标准JSON格式
        config.data = JSON.parse(JSON.stringify(config.data))
        console.log('请求数据规范化后:', config.data)
      } catch (e) {
        console.error('请求数据JSON处理错误:', e)
      }
    }
    
    console.log('发送请求:', {
      url: config.url,
      method: config.method,
      data: config.data,
      headers: config.headers
    })
    return config
  },
  (error) => {
    console.error('请求错误:', error)
    ElMessage.error('请求发送失败')
    return Promise.reject(error)
  }
)

// 响应拦截器
service.interceptors.response.use(
  (response: AxiosResponse) => {
    console.log('Response:', response.data)
    
    // 检查响应状态
    if (!response.data) {
      ElMessage.error('响应数据为空')
      return Promise.reject(new Error('响应数据为空'))
    }

    const { code, msg, data } = response.data

    // 处理成功响应
    if (code === 200) {
      if (msg) {
        ElMessage.success(msg)
      }
      return data
    }

    // 处理业务错误
    ElMessage.error(msg || '操作失败')
    return Promise.reject(new Error(msg || '操作失败'))
  },
  (error) => {
    console.error('Response Error:', error)
    
    // 处理403权限错误
    if (error.response?.status === 403) {
      ElMessage.error('当前角色无权限执行此操作')
      return Promise.reject(error)
    }

    // 处理网络错误
    if (!error.response) {
      ElMessage.error('网络连接失败')
      return Promise.reject(error)
    }

    // 处理其他HTTP错误
    const message = error.response.data?.msg || '请求失败'
    ElMessage.error(message)
    return Promise.reject(error)
  }
)

const request = <T = any>(config: AxiosRequestConfig): Promise<T> => {
  return service(config) as Promise<T>
}

export default request
