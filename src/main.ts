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
import router from '@/route'
import VueHighlightJS from 'vue3-highlightjs'
import { createPinia } from 'pinia'
const pinia = createPinia()


import '@/assets/scss/global.scss'
import 'element-plus/dist/index.css'
import 'highlight.js/styles/atom-one-dark.css'

const app = createApp(App)


app.use(ElementPlus).use(router).use(VueHighlightJS).use(pinia).mount('#app')