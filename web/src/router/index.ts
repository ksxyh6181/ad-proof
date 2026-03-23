import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'

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
    path: '/income',
    component: Layout,
    children: [
      {
        path: '',
        name: 'Income',
        component: () => import('@/views/income/index.vue'),
        meta: { title: '收入门槛证明' }
      }
    ]
  },
  {
    path: '/kyc',
    component: Layout,
    children: [
      {
        path: '',
        name: 'Kyc',
        component: () => import('@/views/kyc/index.vue'),
        meta: { title: 'KYC 等级证明' }
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

router.beforeEach((to, _from, next) => {
  if (typeof to.meta.title === 'string') {
    document.title = `${to.meta.title} · Ad Proof`
  }
  next()
})

export default router
