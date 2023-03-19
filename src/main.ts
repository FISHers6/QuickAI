import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import router from '@/route'

import '@/assets/scss/global.scss'
import 'element-plus/dist/index.css'
// import { useLocalStorage, useMouse, usePreferredDark } from '@vueuse/core'
const app = createApp(App)

app.use(ElementPlus).use(router).mount('#app')