/*
 * @Author: Joe
 * @Date: 2023-03-22 19:00:32
 * @LastEditors: Joe
 * @LastEditTime: 2023-03-23 00:28:20
 * @Description: 
 * 
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 */
import { createApp } from 'vue'
import App from "./App.vue"
import ElementPlus from 'element-plus'
import VueHighlightJS from 'vue3-highlightjs'

import { setupStore } from './store'
import { setupRouter } from './route'

import '@/assets/scss/global.scss'
import '@/assets/scss/index.scss'
import 'element-plus/dist/index.css'
import 'highlight.js/styles/atom-one-dark.css'
// import * as dotenv from 'dotenv';
// dotenv.config();

async function bootstrap() {
    const app = createApp(App)
    setupStore(app)
    await setupRouter(app)
    app.use(ElementPlus)
    app.use(VueHighlightJS)
    app.mount('#app')
}
  
bootstrap()
  

