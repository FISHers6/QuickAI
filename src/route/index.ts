import {
  createRouter,
  createWebHistory,
  createWebHashHistory,
  RouteRecordRaw,
} from 'vue-router'

import Home from '@/views/home/index.vue'
import quickAsk from '@/views/quickAsk/index.vue'
import Prompt from '@/views/prompt/index.vue'
import Chat from '@/views/chatwin/index.vue'
import Test from '@/views/test.vue'
import type { App } from 'vue'
import { setupPageGuard } from './permission'


export const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home/quickask',
  },
  {
    path: '/home',
    component: Home,
    redirect: '/home/quickask',
    meta: {
      title: 'Home',
    },
    children: [
      {
        path: 'quickask',
        component: quickAsk,
        meta: {
          title: '快捷提问',
        },
      },
      {
        path: 'prompt',
        component: Prompt,
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
      {
        path: 'test',
        component: Test,
        meta: {
          title: 'test',
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
