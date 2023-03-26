import {
  createRouter,
  createWebHistory,
  createWebHashHistory,
  RouteRecordRaw,
} from 'vue-router'

import Home from '@/views/home/index.vue'

import ChatGPT4 from '@/views/chatGPT4/index.vue'
import Test from '@/views/testPage/index.vue'

export const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home/chatgpt4',
  },
  {
    path: '/home',
    redirect: '/home/chatgpt4',
  },
  {
    path: '/home',
    component: Home,
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
    ],
  },
]

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

export default new Router().router()
