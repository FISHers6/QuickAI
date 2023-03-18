import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
// import "./styles.css";
import 'element-plus/theme-chalk/dark/css-vars.css'
import { useLocalStorage, useMouse, usePreferredDark } from '@vueuse/core'
const app = createApp(App)

app.use(ElementPlus)
app.mount('#app')