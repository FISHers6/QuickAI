
import { createApp } from "vue";
import App from "@/views/singalChat/index.vue"
import ElementPlus from 'element-plus'
import VueHighlightJS from 'vue3-highlightjs'

import { setupStore } from './store'

import '@/assets/scss/global.scss'
import 'element-plus/dist/index.css'
// import 'highlight.js/styles/atom-one-dark.css'
async function bootstrap() {
    const app = createApp(App)
    setupStore(app)
    app.use(ElementPlus)
    app.use(VueHighlightJS)
    app.mount('#chatWindow')
}
bootstrap()
  

