import {
  createRouter,
  createWebHistory,
  createWebHashHistory,
  RouteRecordRaw,
} from 'vue-router'

import Home from '@/views/home/index.vue'

import ChatGPT4 from '@/views/chatGPT4/index.vue'
import Test from '@/views/testPage/index.vue'
import Chat from '@/views/chatwin/index.vue'

import type { App } from 'vue'
import { setupPageGuard } from './permission'

export const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home/chatgpt4',
  },
  {
    path: '/404',
    name: '404',
    component: () => import('@/views/exception/404/index.vue'),
  },
  {
    path: '/500',
    name: '500',
    component: () => import('@/views/exception/500/index.vue'),
  },
  {
    path: '/home',
    component: Home,
    redirect: '/home/chatgpt4',
    meta: {
      title: 'Home',
    },
    children: [
      {
        path: 'chatgpt4',
        component: ChatGPT4,
        meta: {
          title: 'ChatGPT4.0',
        },
      },
      {
        path: 'prompt',
        component: Test,
        meta: {
          title: '功能场景',
        },
      },
      {
        path: 'chatwin',
        component: Chat,
        meta: {
          title: '对话模式',
        },
      },
    ],
  },
];

class Router {
  router: () => any
  constructor() {
    const creat = createRouter({
      history: createWebHistory('/'),
      routes,
    })
    this.router = () => this.before(creat)
  }

  before(creat: any) {
    creat.beforeEach(async (to: any, from: any, next: any) => {
      next()
    })
    return creat
  }
}

export const router = createRouter({
  history: createWebHashHistory(),
  routes,
  scrollBehavior: () => ({ left: 0, top: 0 }),
})

// setupPageGuard(router)

export async function setupRouter(app: App) {
  app.use(router)
  await router.isReady()
}
