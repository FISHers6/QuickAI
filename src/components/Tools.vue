<template lang='pug'>
#Tools 
  el-button(type="primary" @click="dialogHelpVisible = true") 帮助
  .tools-list
    el-icon.tool-more(color="var(--el-color-primary)" size="24" @click="dialogFormVisible = true")
      Setting
  .tools-list
    el-icon.tool-more(color="var(--el-color-primary)" size="24" @click="closeQuickAskWindow")
      CircleClose
#Setting 
  el-dialog.dialog(v-model="dialogFormVisible" title="设置" class="setting-dialog")
    el-form.dialog-form(:model="form" :label-position="labelPosition")
      el-form-item(label="快捷功能模式" class="setting-form-item")
          el-radio-group(v-model="form.mode")
              el-radio(label="自动输入" class="el-radio-style")
              el-radio(label="对话模式" class="el-radio-style")
              el-radio(label="快捷提问" class="el-radio-style")
      el-form-item(label="夜间模式" :label-width="formLabelWidth" class="setting-form-item")
        .tools-item
          el-switch(v-model="isDarkMode" size="small")
      el-form-item(label="连续对话" :label-width="formLabelWidth" class="setting-form-item")
        .tools-item
          el-switch(v-model="form.useChatContenxt" size="small")
      el-form-item(label="划词功能(Win10+)" :label-width="formLabelWidth" class="setting-form-item")
        .tools-item
          el-switch(v-model="form.enableSelect" inline-prompt active-text="开启" inactive-text="关闭")  
      el-form-item(label="API KEY (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.api_key" autocomplete="off" placeholder="输入API KEY, 速度更快~" class="setting-form")
      el-form-item(label="Proxy 代理 (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.proxy" autocomplete="off" placeholder="代理地址" class="setting-form")
      el-form-item(label="Prompt 预设 (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.systemMessage" autocomplete="off" placeholder="想让AI扮演什么角色~ 请输入提示词" class="setting-form")
      el-form-item(label="搜索快捷键" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.searchShortcut" autocomplete="off" placeholder="请输入搜索快捷键, 默认Shift+Space" class="setting-form")
      el-form-item(label="选中文本快捷提问" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.quickAskShortcut" autocomplete="off" placeholder="请输入提问快捷键, 默认Shift+D" class="setting-form")
      el-form-item(label="选中文本快捷对话" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.chatShortcut" autocomplete="off" placeholder="请输入对话快捷键, 默认Shift+C" class="setting-form")
      //- el-form-item(label="语言" :label-width="formLabelWidth" class="setting-form-item")
      //-   el-select(v-model="form.language" placeholder="Please select a zone")
      //-     el-option( label="中文" value="zh")
      //-     el-option( label="英文" value="en")
    template(#footer)
      span.dialog-footer
        el-button(@click="dialogFormVisible = false" class="cancel-button") 取消
        el-button(type="primary" @click="submitSetting") 确认
#Help
  el-dialog.help-dialog(v-model="dialogHelpVisible" title="帮助" center)
    .help-text 请先点击设置页面, 添加自己的Open API Key
    .help-text 内测版无需API Key, 请加
      a(href ='http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=Vzl1HA8jx9eXRsYyKxxhhIs8CGC7R5eN&authKey=ZXZSqYCCT6i0SM5Jypg1aR%2F2d1uLqryhuhkckKrPUTieQuWRFQ%2B2zd6dg%2FSc4uc3&noverify=0&group_code=456730400') QQ交流群456730400
      span 下载(免费)
    .help-text 搜索快捷键: 默认键是"Shift+Space"
    .help-text 选中文本快捷提问: 默认快捷键是"Shift+D"
    .help-text 选中文本快捷对话: 默认快捷键是"Shift+C"
    .help-text 划词功能仅支持windws版本
    .help-text 使用软件时有问题欢迎在
      a(href="https://github.com/FISHers6/QuickAI") github
      span 提Issue, 或在交流群中提问~
</template>

<script lang='ts' setup>
import { MoreFilled, CircleClose, Setting } from '@element-plus/icons-vue'
import { reactive, ref } from 'vue'
import { useSettings } from '@/hooks/useSettings'
import { invoke } from '@tauri-apps/api'
import type { SettingsState, AppConfig } from '@/store/modules/settings/helper'

const { updateSetting, getSetting } = useSettings()
const labelPosition = ref('left')
const dialogFormVisible = ref(false)
const dialogHelpVisible = ref(false)
// not used
const formLabelWidth = '140px'
const changeMode = (val: boolean) => {
  document.documentElement.classList[ val ? 'add' : 'remove' ]('theme-dark')
}

const changeDialogMode = (val: boolean) => {
  const dialogEl = document.querySelector('.el-dialog__wrapper');
  if (dialogEl) {
    dialogEl.classList[val ? 'add' : 'remove']('theme-dark');
  }
};

const currentSetting = getSetting();
const form = reactive({
  api_key: currentSetting.apiKey,
  proxy: currentSetting.proxy,
  useChatContenxt: currentSetting.useChatContext,
  language: currentSetting.language,
  systemMessage: currentSetting.systemMessage,
  searchShortcut: currentSetting.searchShortcut,
  enableSelect: currentSetting.enableSelect,
  quickAskShortcut: currentSetting.quickAskShortcut,
  chatShortcut: currentSetting.chatShortcut,
  mode: currentSetting.mode,
})

const isDarkMode = ref(currentSetting.isDarkMode)

watch(isDarkMode, changeMode, { immediate: true });
watch(isDarkMode, changeDialogMode, { immediate: true });

const submitSetting = () => {
  dialogFormVisible.value = false
  let settings = {
    apiKey: form.api_key,
    proxy: form.proxy,
    useChatContenxt: form.useChatContenxt,
    language: form.language,
    systemMessage: form.systemMessage,
    isDarkMode: isDarkMode.value,
    searchShortcut: form.searchShortcut,
    enableSelect: form.enableSelect,
    quickAskShortcut: form.quickAskShortcut,
    chatShortcut: form.chatShortcut,
    mode: form.mode,
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
  };
  // 修改配置
  invoke('update_app_config',  { payload: appConfig})
  // 修改快捷键
  invoke('update_shortcut')
}

const closeQuickAskWindow = () => {
  invoke('close_window')
}

</script>
<style lang='scss' scoped>
#Tools {
  display: flex;
  align-items: center;

  &>* {
    margin-right: 1rem;

    &:last-child {
      margin: 0 !important;
    }
  }

  :deep(.tool-more) {
    cursor: pointer;

    &:active {
      color: var(--el-color-primary-dark-2) !important;
    }
  }

  :deep(el-button.more) {
    font-size: 36px !important;
  }

}

#Setting {
  // not work good, need someone help to fix it
  // background: var(--el-text-color-primary) !important;
  // .dialog {
  //   background: var(--el-text-color-primary) !important;
  // }
  // .dialog-form {
  //   background: var(--el-text-color-primary);
  // }
  .el-select {
    width: 300px;
  }
  .el-input {
    width: 300px;
  }
  .dialog-footer button:first-child {
    margin-right: 10px;
  }
}

:deep(.help-text) {
  color: black !important;
  margin-top: 10px;
}

:deep(.el-radio-style) {
  color: black !important;
}
:deep(.setting-form .el-form-item__label) {
  color: black !important;
}

:deep(.setting-form .el-input__inner) {
  color: black !important;
}

:deep(.cancel-button) {
  color: black !important;
}

:deep(.setting-form-item .el-form-item__label) {
  color: black !important;
}

:deep(.setting-dialog) {
  --el-dialog-width: 80% !important;
}

:deep(.help-dialog) {
  --el-dialog-width: 80% !important;
}
</style>