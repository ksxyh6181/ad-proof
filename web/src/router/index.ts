import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'
import { h } from 'vue'
import { RouterView } from 'vue-router'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: Layout,
    redirect: '/home',
    children: [
      {
        path: 'home',
        name: 'Home',
        component: () => import('@/views/home/index.vue'),
        meta: { title: '首页' }
      }
    ]
  },
  {
    path: '/credential',
    component: Layout,
    redirect: '/credential/issue',
    meta: { title: '学历凭证系统' },
    children: [
      {
        path: 'issue',
        name: 'Issue',
        component: () => import('@/views/credential/issue.vue'),
        meta: { title: '颁发证书' }
      },
      {
        path: 'verify',
        name: 'Verify',
        component: () => import('@/views/credential/verify.vue'),
        meta: { title: '验证证书' }
      },
      {
        path: 'get',
        name: 'Get',
        component: () => import('@/views/credential/get.vue'),
        meta: { title: '获取证书' }
      },
      {
        path: 'blockchain',
        name: 'Blockchain',
        component: () => import('@/views/credential/blockchain.vue'),
        meta: { title: '区块链凭证' }
      }
    ]
  },
  {
    path: '/financial',
    component: Layout,
    redirect: '/financial/index',
    meta: { title: '金融凭证系统' },
    children: [
      {
        path: 'index',
        name: 'FinancialIndex',
        component: () => import('@/views/financial/index.vue'),
        meta: { title: '金融凭证首页' }
      },
      {
        path: 'individual',
        name: 'Individual',
        component: { render: () => h(RouterView) },
        redirect: '/financial/individual/list',
        meta: {
          title: '个人用户',
          icon: 'el-icon-user',
          roles: ['individual']
        },
        children: [
          {
            path: 'list',
            name: 'IndividualList',
            component: () => import('@/views/financial/individual/list.vue'),
            meta: {
              title: '我的凭证',
              icon: 'el-icon-collection',
              roles: ['individual']
            }
          }
        ]
      },
      {
        path: 'institution',
        name: 'FinancialInstitution',
        component: { render: () => h(RouterView) },
        redirect: '/financial/institution/issue-income',
        meta: {
          title: '金融机构',
          icon: 'el-icon-office-building',
          roles: ['financial_institution']
        },
        children: [
          {
            path: 'issue-income',
            name: 'IssueIncome',
            component: () => import('@/views/financial/institution/issue-income.vue'),
            meta: {
              title: '颁发收入证明',
              icon: 'el-icon-wallet',
              roles: ['financial_institution']
            }
          },
          {
            path: 'issue-credit',
            name: 'IssueCredit',
            component: () => import('@/views/financial/institution/issue-credit.vue'),
            meta: {
              title: '颁发信用评分',
              icon: 'el-icon-star-on',
              roles: ['financial_institution']
            }
          },
          {
            path: 'issue-cross-border',
            name: 'IssueCrossBorder',
            component: () => import('@/views/financial/institution/issue-cross-border.vue'),
            meta: {
              title: '颁发跨境信用',
              icon: 'el-icon-map-location',
              roles: ['financial_institution']
            }
          }
        ]
      },
      {
        path: 'verifier',
        name: 'FinancialVerifier',
        component: { render: () => h(RouterView) },
        redirect: '/financial/verifier/verify',
        meta: {
          title: '验证方',
          icon: 'el-icon-check',
          roles: ['verifier']
        },
        children: [
          {
            path: 'verify',
            name: 'FinancialVerify',
            component: () => import('@/views/financial/verifier/verify.vue'),
            meta: {
              title: '验证凭证',
              icon: 'el-icon-key',
              roles: ['verifier']
            }
          }
        ]
      }
    ]
  },
  {
    path: '/:pathMatch(.*)*',
    redirect: '/'
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 全局前置守卫
router.beforeEach((to, from, next) => {
  // 获取导航目标的路由配置
  const matchedRoutes = to.matched
  // 检查是否需要特定角色
  const requiresRole = matchedRoutes.some(record => record.meta.roles && record.meta.roles.length > 0)
  
  if (requiresRole) {
    // 获取当前角色
    const currentRole = localStorage.getItem('role') || ''
    
    // 检查角色是否有权访问
    const hasPermission = matchedRoutes.every(record => {
      if (record.meta.roles && record.meta.roles.length > 0) {
        return record.meta.roles.includes(currentRole)
      }
      return true
    })
    
    if (hasPermission) {
      next()
    } else {
      // 无权访问，重定向到相应的默认页面
      if (to.path.startsWith('/financial')) {
        // 设置正确的角色
        if (to.path.includes('/financial/institution')) {
          localStorage.setItem('role', 'financial_institution')
          next({ path: '/financial/index' })
        } else if (to.path.includes('/financial/individual')) {
          localStorage.setItem('role', 'individual')
          next({ path: '/financial/index' })
        } else if (to.path.includes('/financial/verifier')) {
          localStorage.setItem('role', 'verifier')
          next({ path: '/financial/index' })
        } else {
          next({ path: '/financial/index' })
        }
      } else if (to.path.startsWith('/credential')) {
        // 处理证书系统权限
        // ... 可以根据需要添加具体逻辑
        next({ path: '/credential' })
      } else {
        next({ path: '/' })
      }
    }
  } else {
    // 不需要特定角色的路由，直接放行
    next()
  }
})

export default router
