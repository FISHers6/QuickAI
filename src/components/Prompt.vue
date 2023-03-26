<template lang="pug">
el-autocomplete(v-model="state" :fetch-suggestions="querySearch" placeholder="Ask ChatGPT. Ex: Write an email reply in yoda style" @select="handleSelect" @input="handleInput")
      template(#suffix)
        el-icon.el-input__icon(@click="handleIconClick")
          search
      template(#default="{ item }")
        div.title {{ item.title }}
        span.description {{ item.description }}
.prompt-card-container
    .prompt-card(v-for="(item, index) in displayItems" :key="index" :span="8" :offset="index > 0 ? 2 : 0" :class="{ active: isSelected(item) }" @click="handleSelectItem(item)")
        el-card.el-card-style(:body-style="{ padding: '0px' }")
            el-icon.prompt-icon(size="35")
              component(:is="item.icon")
            .prompt-title
                span {{ item.title }}
</template>
  
<script lang='ts' setup>
import { ref } from 'vue'
import { usePromptModeStore } from '@/hooks/store'
import { Edit, Promotion, Search } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'


const promptModeStore = usePromptModeStore()
const items = computed(() => promptModeStore.items)
const state = ref('')
const selected = ref<LinkItem | null>(null)

interface LinkItem {
  title: string
  description: string
  icon: any
}

const querySearch = (queryString: string, cb: any) => {
  const results = queryString
    ? items.value.filter(createFilter(queryString))
    : items.value
  console.log(results)
  cb(results)
}

const createFilter = (queryString: any) => {
  return (restaurant: any) => {
    return (
      restaurant.title.toLowerCase().indexOf(queryString.toLowerCase()) === 0 
      || restaurant.description.toLowerCase().indexOf(queryString.toLowerCase()) === 0
    )
  }
}

const handleInput = (value: string) => {
    selected.value = null
    state.value = value
}

const handleSelect = (item: any) => {
  selected.value = item
}

const handleIconClick = (ev: Event) => {
  console.log(ev)
}

// 计算展示的链接列表
const displayItems = computed(() => {
  if (state.value) {
    // 有搜索内容时，展示搜索结果
    return items.value.filter(createFilter(state.value))
  } else if (selected.value) {
    state.value = selected.value.title
    // 有选中的项时，只展示该项
    return [selected.value]
  } else {
    // 否则，展示所有项
    return items.value
  }
})


const isSelected = (item: any) => {
    return promptModeStore.selectedPrompt && promptModeStore.selectedPrompt.title === item.title
}

const handleSelectItem = (item: any) => {
    if (isSelected(item)) {
        promptModeStore.clearSelectedPrompt()
        closePromptNotification(item.title)
    }else {
        promptModeStore.setSelectedPrompt(item.title)
        openPromptNotification(item.title)
    }
}

const openPromptNotification = (mode: string) => {
  ElMessage({
    message: 'enable ' + mode,
    type: 'success',
  })
}

const closePromptNotification = (mode: string) => {
  ElMessage({
    message: 'close ' + mode,
    type: 'warning',
  })
}

</script>

<style>

.prompt-card-container {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    width: 100%;
    margin-top: 10px;
}

.prompt-card {
    justify-content: center;
    display: flex;
    flex-direction: column;
    width: 140px;
    height: 105px;
    margin: 10px 10px;
    align-items: center;
}

.prompt-icon {
    padding-top: 10px;
}

.prompt-title {
    padding: 5px 10px 12px 10px;
    overflow: hidden;
    text-overflow: ellipsis;
}

.el-card-style {
    justify-content: center;
    text-align: center;
    width: 140px;
    height: 120px;
}

.description {
    overflow: hidden;
    text-overflow: ellipsis;
    width: 80%;
}

.prompt-card.active {
  background-color: #e1f5fe;
}


</style>
  