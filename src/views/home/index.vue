
<template lang="pug">
#Home
  Layouts
  //- el-container.bg-slate-50
    el-header.header-border
      el-menu.menu-style(:default-active='activeIndex', mode='horizontal', :ellipsis='false', @select='handleMenuSelect')
        el-menu-item(index='0') ChatGPT 4.0
        //- .flex-grow
        el-menu-item(index='1') Edit Prompts
        el-sub-menu(index='2')
          template(#title) More
          el-menu-item(index='2-1') API Key
          el-sub-menu(index='2-2')
            template(#title) Dark
            el-menu-item(index='2-2-1') Light
            el-menu-item(index='2-2-2') Dark
    //- el-main
      .textarea-container
        .textarea-tip Your selected context is copied below:
        el-row(justify='start')
          el-input.textarea-bg(v-model='textarea', :autosize='{ minRows: 2, maxRows: 4 }', type='textarea', placeholder='Your selected content').
      .prompt-container.border-slate-300.rounded-lg
        el-row(justify='start')
          el-col
            el-autocomplete(v-model='state1', :fetch-suggestions='querySearch', clearable, placeholder='Ask ChatGPT. Ex: Write an email reply in yoda style', @select='handleSelect').
        el-row(justify='start')
          el-col.prompt-button-container
            //- el-button.prompt-button(v-for='button in buttons', :key='button.text', :type='button.type', text, bg) {{ button.text }}
      .result-container
        el-row.say-tip(justify='space-between')
          el-col(:span='20') ChatGPT Says:
          el-col(:span='4')
            el-icon
              Refresh
            el-icon
              CopyDocument
        el-row(justify='start')
          //  <el-empty description="description" /> 
          el-skeleton(v-if='loading', :rows='4', animated)
          el-text.mx-1.result-text(v-else) {{response}}
  </template>
  
  
  
<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { Edit, Search, Share, Setting, CopyDocument, Refresh, SetUp } from '@element-plus/icons-vue'
import Layouts from '@/layouts/index.vue'

interface RestaurantItem {
  value: string
  link: string
}

const state1 = ref('')
const textarea = ref('')
const loading = ref(true)
const response = ref('Type your request above and press enter')

const activeIndex = ref('1')
const handleMenuSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}

const buttons = [
  { type: '', text: 'plain a gpt name' },
  { type: 'primary', text: 'primary  a gpt name' },
  { type: 'success', text: 'success' },
  { type: 'info', text: 'info' },
  { type: 'warning', text: 'warning' },
  { type: 'danger', text: 'danger  a gpt name' },
] as const

const restaurants = ref<RestaurantItem[]>([])
const querySearch = (queryString: string, cb: any) => {
  const results = queryString
    ? restaurants.value.filter(createFilter(queryString))
    : restaurants.value
  // call callback function to return suggestions
  cb(results)
}
const createFilter = (queryString: string) => {
  return (restaurant: RestaurantItem) => {
    return (
      restaurant.value.toLowerCase().indexOf(queryString.toLowerCase()) === 0
    )
  }
}
const loadAll = () => {
  return [
    { value: 'vue', link: 'https://github.com/vuejs/vue' },
    { value: 'element', link: 'https://github.com/ElemeFE/element' },
    { value: 'cooking', link: 'https://github.com/ElemeFE/cooking' },
    { value: 'mint-ui', link: 'https://github.com/ElemeFE/mint-ui' },
    { value: 'vuex', link: 'https://github.com/vuejs/vuex' },
    { value: 'vue-router', link: 'https://github.com/vuejs/vue-router' },
    { value: 'babel', link: 'https://github.com/babel/babel' },
  ]
}

const handleSelect = (item: RestaurantItem) => {
  console.log(item)
}

onMounted(() => {
  restaurants.value = loadAll()
})

</script>
  
<style lang="scss" scoped>
.menu-style {
  border-bottom: 0px;
  --tw-bg-opacity: 1;
}

.el-main {
  height: 100%;
}

.el-row {
  margin-bottom: 5px;
}

.el-row:last-child {
  margin-bottom: 0;
}

.bg-slate-50 {
  --tw-bg-opacity: 1;
}

.header-border {
  border-bottom-width: 1px;
}

.textarea-bg {
  --tw-bg-opacity: 1;
}

.textarea-bg {
  margin-top: 5px;
}

.prompt-container {
  display: flex;
  padding-left: 8px;
  padding-right: 8px;
  padding-top: 8px;
  padding-bottom: 8px;
  overflow: hidden;
  justify-content: space-between;
  flex-direction: column;
  border-width: 1px;
}

.textarea-container {
  padding-left: 1px;
  padding-right: 1px;
  padding-top: 10px;
  padding-bottom: 10px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: stretch;

}

.textarea-tip {
  --tw-text-opacity: 1;
  font-weight: 400;
  font-size: 14px;
  line-height: 20px;
  padding-left: 2px;
  padding-right: 2px;
}

.result-container {
  border-top-width: 1px;
  --tw-border-opacity: 1;
  margin-top: 10px;
  padding-top: 12px;
  padding-bottom: 8px;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: stretch;
}

.say-tip {
  font-weight: 500;
  font-size: 18px;
  line-height: 28px;
  --tw-text-opacity: 1;
  margin-bottom: 5px;
}

.result-text {
  padding-left: 2px;
  font-size: 15px;
  line-height: 24px;
}

.copy-icon {
  margin-left: 6px;
}

.border-slate-300 {
  --tw-border-opacity: 1;
  border-color: rgb(203 213 225/var(--tw-border-opacity));
}

.rounded-lg {
  border-radius: 8px;
}

*,
:before,
:after {
  box-sizing: border-box;
  border: 0 solid #e5e7eb;
}


.border-line {
  box-sizing: border-box;
  border: 0 solid #e5e7eb;
}

:deep(.el-autocomplete) {
  width: 100%;
}

.prompt-button-container {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  flex-wrap: wrap;
  margin: 0 -13px;
}

.prompt-button {
  margin-bottom: 5px;
}

.prompt-button:first-of-type {
  margin-left: 12px;
}

.flex-grow {
  flex-grow: 1;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}</style>
  