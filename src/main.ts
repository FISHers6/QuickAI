/*
 * @Author: Joe
 * @Date: 2023-03-22 19:00:32
 * @LastEditors: Joe
 * @LastEditTime: 2023-03-23 00:28:20
 * @FilePath: /ChatGPT-PC/src/main.ts
 * @Description: 
 * 
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 */
import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import VueHighlightJS from 'vue3-highlightjs'
import { createPinia } from 'pinia'

import { setupI18n } from './locales'
import { setupAssets, setupScrollbarStyle } from './plugins'
import { setupStore } from './store'
import { setupRouter } from './route'

import '@/assets/scss/global.scss'
import 'element-plus/dist/index.css'
import 'highlight.js/styles/atom-one-dark.css'

async function bootstrap() {
    const app = createApp(App)
    setupAssets()

    setupScrollbarStyle()

    setupStore(app)

    setupI18n(app)

    await setupRouter(app)
    app.use(ElementPlus)
    app.use(VueHighlightJS)

    app.mount('#app')
}
  
bootstrap()
  

