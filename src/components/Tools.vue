<template lang='pug'>
#Tools 
  el-button(type="primary") Plus
  .tools-list
    el-icon.tool-more(color="var(--el-color-primary)" size="24" @click="dialogFormVisible = true")
      Setting
  .tools-list
    el-icon.tool-more(color="var(--el-color-primary)" size="24" @click="closeQuickAskWindow")
      CircleClose
#Setting 
  el-dialog.dialog(v-model="dialogFormVisible" title="设置" class="setting-dialog")
    el-form.dialog-form(:model="form" :label-position="labelPosition")
      el-form-item(label="夜间模式" :label-width="formLabelWidth" class="setting-form-item")
        .tools-item
          el-switch(v-model="isDarkMode" size="small")
      el-form-item(label="连续对话" :label-width="formLabelWidth" class="setting-form-item")
        .tools-item
          el-switch(v-model="form.useChatContenxt" size="small")
      el-form-item(label="API KEY (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.api_key" autocomplete="off" placeholder="输入API KEY, 速度更快~" class="setting-form")
      el-form-item(label="Proxy 代理 (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.proxy" autocomplete="off" placeholder="代理地址" class="setting-form")
      el-form-item(label="Prompt 预设 (可选)" :label-width="formLabelWidth" class="setting-form-item")
        el-input(v-model="form.systemMessage" autocomplete="off" placeholder="想让AI扮演什么角色~ 请输入提示词" class="setting-form")
      //- el-form-item(label="语言" :label-width="formLabelWidth" class="setting-form-item")
      //-   el-select(v-model="form.language" placeholder="Please select a zone")
      //-     el-option( label="中文" value="zh")
      //-     el-option( label="英文" value="en")
    template(#footer)
      span.dialog-footer
        el-button(@click="dialogFormVisible = false" class="cancel-button") 取消
        el-button(type="primary" @click="submitSetting") 确认
</template>
<script lang='ts' setup>
import { MoreFilled, CircleClose, Setting } from '@element-plus/icons-vue'
import { reactive, ref } from 'vue'
import { useSettings } from '@/hooks/useSettings'
import { invoke } from '@tauri-apps/api'
import type { SettingsState } from '@/store/modules/settings/helper'

const { updateSetting, getSetting } = useSettings()
const labelPosition = ref('left')
const dialogFormVisible = ref(true)
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
  }
  updateSetting(settings)
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
</style>