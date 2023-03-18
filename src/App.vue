
<template>
  <div class="common-layout">
    <el-container class="bg-slate-50">
      <el-header class="header-border">
        <el-row :gutter="20" justify="end">
          <!-- <el-col :span="10" :offset=4>
            <div class="grid-content ep-bg-purple"/>ChatGPT Translate
          </el-col> -->
            <el-col :span="10">
              <el-button type="primary" plain :icon="Edit" size="small" color="#626aef">Edit Prompts</el-button>
              <el-button type="primary" plain :icon="Setting" size="small" color="#626aef">Settings</el-button>
              <!-- <el-button type="primary" plain :icon="Share" size="small" color="#626aef" :dark="isDark" /> -->
          </el-col>
        </el-row>
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

        <div class="result-container">
          <el-row justify="space-between" class="say-tip">
            <el-col :span="5">ChatGPT Says:</el-col>
            <el-col :span="2">
              <el-icon><Refresh /></el-icon>
              <el-icon class="copy-icon"><CopyDocument /></el-icon>
            </el-col>
          </el-row>
          <el-row justify="start">
            <!-- <el-empty description="description" /> -->
            <el-skeleton v-if="loading" :rows="4" animated/>
            <el-text v-else class="mx-1 result-text">{{response}}</el-text>
          </el-row>
        </div>
      </el-main>
    </el-container>
  </div>
</template>



<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { Edit, Search, Share, Setting, CopyDocument, Refresh} from '@element-plus/icons-vue'
import { onMounted, ref } from 'vue'

interface RestaurantItem {
  value: string
  link: string
}

const state1 = ref('')
const textarea = ref('')
const loading = ref(false)
const response = ref('Type your request above and press enter')
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

.el-header {
  height: 40px;
  line-height: 40px;
}

.el-main {
  height: 520px;
}

.el-row {
  margin-bottom: 5px;
}
.el-row:last-child {
  margin-bottom: 0;
}

.bg-slate-200 {
  --tw-bg-opacity: 1;
  background-color: rgb(226 232 240/var(--tw-bg-opacity));
}

.bg-slate-50 {
  --tw-bg-opacity: 1;
  background-color: rgb(248 250 252/var(--tw-bg-opacity));
}


.header-border {
  border-bottom-width: 1px;
  --tw-border-opacity: 1;
  border-bottom-color: rgb(203 213 225/var(--tw-border-opacity));
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

.result-container {
  border-top-width: 1px;
  --tw-border-opacity: 1;
  border-top-color: rgb(203 213 225/var(--tw-border-opacity));
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
  color: rgb(100 116 139/var(--tw-text-opacity));
  margin-bottom: 5px;
}

.result-text {
  padding-left:2px;
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

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
