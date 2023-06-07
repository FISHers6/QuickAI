<template lang="pug">
.setting-container
    el-form(:model="form" label-width="120px" class="setting-form")
        el-form-item(label="快捷功能模式" :label-width="formLabelWidth")
            el-radio-group(v-model="form.mode")
                el-radio(label="自动输入")
                el-radio(label="对话模式")
                el-radio(label="快捷提问")
        el-form-item(label="夜间模式" :label-width="formLabelWidth")
            .tools-item
                el-switch(v-model="form.isDarkMode" size="small")
        el-form-item(label="连续对话" :label-width="formLabelWidth")
            .tools-item
                el-switch(v-model="form.useChatContenxt" size="small")
        el-form-item(label="划词功能(Win10+)" :label-width="formLabelWidth" class="setting-form-item")
          .tools-item
            el-switch(v-model="form.enableSelect" inline-prompt active-text="开启" inactive-text="关闭")
        el-form-item(label="记忆对话数量" :label-width="formLabelWidth" class="setting-form-item")
          el-input(v-model="form.messageContextCount" autocomplete="off" placeholder="请输入多轮对话记忆的上下文数量" class="setting-form")
        el-form-item(label="API KEY (可选)" :label-width="formLabelWidth")
            el-input(v-model="form.api_key" autocomplete="off" placeholder="输入自己的API KEY, 速度更快~")
        el-form-item(label="Proxy 代理 (可选)" :label-width="formLabelWidth")
            el-input(v-model="form.proxy" autocomplete="off" placeholder="设置代理地址")
        el-form-item(label="Prompt 预设 (可选)" :label-width="formLabelWidth")
            el-input(v-model="form.systemMessage" autocomplete="off" placeholder="想让AI扮演什么角色~ 请输入提示词")
        el-form-item(label="搜索快捷键" :label-width="formLabelWidth" class="setting-form-item")
          el-input(v-model="form.searchShortcut" autocomplete="off" placeholder="请输入搜索快捷键, 默认Shift+Space" class="setting-form")
        el-form-item(label="选中文本快捷提问" :label-width="formLabelWidth" class="setting-form-item")
          el-input(v-model="form.quickAskShortcut" autocomplete="off" placeholder="请输入提问快捷键, 默认Shift+D" class="setting-form")
        el-form-item(label="选中文本快捷对话" :label-width="formLabelWidth" class="setting-form-item")
          el-input(v-model="form.chatShortcut" autocomplete="off" placeholder="请输入对话快捷键, 默认Shift+C" class="setting-form")
        //- el-form-item(label="语言" :label-width="formLabelWidth")
        //-     el-select(v-model="form.language" placeholder="Please select a zone")
        //-         el-option( label="中文" value="zh")
        //-         el-option( label="英文" value="en")
</template>
  
<script lang="ts" setup>
import { reactive, watch } from 'vue'
import _ from 'lodash'

import { useSettings } from '@/hooks/useSettings'
import { invoke } from '@tauri-apps/api'
import { ElMessage } from 'element-plus'
import { fromJSON } from 'postcss'
import type { SettingsState, AppConfig } from '@/store/modules/settings/helper'

const { updateSetting, getSetting } = useSettings()

const changeMode = (val: boolean) => {
  document.documentElement.classList[ val ? 'add' : 'remove' ]('theme-dark')
}
const currentSetting = getSetting();
const form = reactive({
  mode: currentSetting.mode,
  api_key: currentSetting.apiKey,
  proxy: currentSetting.proxy,
  useChatContenxt: currentSetting.useChatContext,
  language: currentSetting.language,
  systemMessage: currentSetting.systemMessage,
  isDarkMode: currentSetting.isDarkMode,
  quickAskShortcut: currentSetting.quickAskShortcut,
  searchShortcut: currentSetting.searchShortcut,
  chatShortcut: currentSetting.chatShortcut,
  enableSelect: currentSetting.enableSelect,
  messageContextCount: currentSetting.messageContextCount
})

// 监听form对象
watch(form, (newValue, oldValue) => {
  debouncedSubmitSetting()
}, { deep: true })

const debouncedSubmitSetting = _.debounce(() => {
  let settings = {
    apiKey: form.api_key,
    proxy: form.proxy,
    useChatContenxt: form.useChatContenxt,
    language: form.language,
    systemMessage: form.systemMessage,
    isDarkMode: form.isDarkMode,
    mode: form.mode,
    quickAskShortcut: form.quickAskShortcut,
    searchShortcut: form.searchShortcut,
    chatShortcut: form.chatShortcut,
    enableSelect: form.enableSelect,
    messageContextCount: form.messageContextCount,
  }
  updateSetting(settings)
  let appConfig: AppConfig = {
    quickAskShortcut: settings.quickAskShortcut,
    searchShortcut: settings.searchShortcut,
    chatShortcut: settings.chatShortcut,
    mode: settings.mode,
    isDarkMode: settings.isDarkMode,
    language: settings.language,
    apiKey: settings.apiKey,
    proxy: settings.proxy,
    useChatContext: settings.useChatContenxt,
    enableSelect: settings.enableSelect,
    messageContextCount: settings.messageContextCount,
  };
  // 修改配置
  invoke('update_app_config',  { payload: appConfig})
  // 修改快捷键
  invoke('update_shortcut')
  ElMessage({
    message: '修改成功' ,
    type: 'success',
  })
}, 800) // 在这里设置了一个500毫秒的时间窗口

const closeQuickAskWindow = () => {
  invoke('close_window')
}

</script>
  
<style scoped>
.setting-container {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  width: 100%;
  position: absolute;
}
</style>