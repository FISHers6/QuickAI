
<template>
  <div class="common-layout">
    <el-container class="bg-slate-50">
      <el-header class="header-border">
        <el-menu
        :default-active="activeIndex"
        class="menu-style"
        mode="horizontal"
        :ellipsis="false"
        @select="handleMenuSelect"
      >
        <el-menu-item index="0">ChatGPT 4.0</el-menu-item>
        <div class="flex-grow" />
        <el-menu-item index="1">Edit Prompts</el-menu-item>
        <el-sub-menu index="2">
          <template #title>More</template>
          <el-menu-item index="2-1">API Key</el-menu-item>
          <el-sub-menu index="2-2">
            <template #title>Mode</template>
            <el-menu-item index="2-2-1">Light</el-menu-item>
            <el-menu-item index="2-2-2">Dark</el-menu-item>
          </el-sub-menu>
        </el-sub-menu>
      </el-menu>
      </el-header>
      <el-main>
        <div class="textarea-container">
          <div class="textarea-tip">
            Your selected context is copied below:
          </div>
        <el-row justify="start">
          <el-input
          class="textarea-bg"
          v-model="textarea"
          :autosize="{ minRows: 2, maxRows: 4 }"
          type="textarea"
          placeholder="Your selected content"
        />
        </el-row>
        </div>

        <div class="prompt-container border-slate-300 rounded-lg">
          <el-row justify="start">
            <el-col>
              <el-autocomplete
                v-model="state1"
                :fetch-suggestions="querySearch"
                clearable
                placeholder="Ask ChatGPT. Ex: Write an email reply in yoda style"
                @select="handleSelect"
              />
            </el-col>
          </el-row>
          <el-row justify="start">
            <el-col class="prompt-button-container">
              <el-button
              class="prompt-button"
              v-for="button in buttons"
              :key="button.text"
              :type="button.type"
              text
              bg
              >{{ button.text }}</el-button
            >
            </el-col>
          </el-row>
        </div>
        <Result :loading=loading :response=response></Result>
      </el-main>
    </el-container>
  </div>
</template>



<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted, ref} from 'vue'
import Result from './components/Result.vue'

interface RestaurantItem {
  value: string
  link: string
}

const state1 = ref('')
const textarea = ref('')
const loading = ref(false)
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

<style scoped>
.menu-style {
  border-bottom: 0px;
  --tw-bg-opacity: 1;
  background-color: rgb(248 250 252/var(--tw-bg-opacity));
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
  background-color: rgb(248 250 252/var(--tw-bg-opacity));
}

.header-border {
  border-bottom-width: 1px;
}
.textarea-bg {
  --tw-bg-opacity: 1;
  background-color: rgb(226 232 240/var(--tw-bg-opacity));
}
.textarea-bg {
  margin-top: 5px;
}
.prompt-container{
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
  color: rgb(100 116 139/var(--tw-text-opacity));
  font-weight: 400;
  font-size: 14px;
  line-height: 20px;
  padding-left: 2px;
  padding-right: 2px;
}

.border-slate-300 {
  --tw-border-opacity: 1;
  border-color: rgb(203 213 225/var(--tw-border-opacity));
}

.rounded-lg {
  border-radius: 8px;
}

*, :before, :after {
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
  display: inline-block;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
