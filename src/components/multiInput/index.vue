<template lang='pug'>
#multi-input.searchInput
  el-input(v-model="inputValue" placeholder="请输入你的问题, 可以添加图片文档哦~" @dblclick="openUpload" @keyup.enter.prevent="onEnterPress")
    template(#prefix)
      el-icon.prefix-icon
        Search
      #file-list
        el-tag(v-for="item in fileList" :key="item" effect="plain" closable @close="handleClose(item)") {{ handleType(item.raw.type) }} | {{ item.name }}
    template(#suffix)
      el-icon.el-input__icon.point-send-icon(@click="onEnterPress")
        if loading
          Loading/
        else
          Promotion/
el-row.searchTab
  el-col(:span="16")
    el-radio-group(v-model="modeSelect" size="small")
      el-radio-button(label="自动输入" :class="{ 'mode-active': modeSelect.value === '自动输入' }" @click="updateMode('自动输入')") 自动输入
      el-radio-button(label="对话模式" :class="{ 'mode-active': modeSelect.value === '对话模式' }" @click="updateMode('对话模式')") 对话模式
      el-radio-button(label="快捷提问" :class="{ 'mode-active': modeSelect.value || modeSelect.value === '快捷提问' }" @click="updateMode('快捷提问')") 快捷提问  
  el-col(:span="8")   
    el-upload(
      ref="uploadRef"
      v-model:file-list="fileList"
      class="upload-demo"
      action="#"
      :on-change="onChange"
      :auto-upload="false"
      :show-file-list="false")
      span.add-file 文件/图片
</template>
<script lang='ts' setup>
import { Search } from '@element-plus/icons-vue'
import { remove, cloneDeep } from 'lodash'
import { getFile, getFileMeta} from "@/hooks/useFile"
import type FileMeta from "@/hooks/useFile"
import type MultiInputValue from "@/typings/input"
import Loading from "@/components/loading.vue"
import { Promotion } from "@element-plus/icons-vue"
import { useSettings } from '@/hooks/useSettings'
import type { SettingsState } from '@/store/modules/settings/helper'

const { updateSetting, getSetting } = useSettings()

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  mode: {
    type: String,
    default: '快捷提问',
  },
});
const fileList: any = ref([])

const handleClose = (item: any) => fileList.value = remove(fileList.value, (val: any) => val.uid !== item.uid)

const handleType = (type: string): string => {
  const fileType: any = {
    'text/plain': 'TXT',
    'application/pdf': 'PDF',
    'application/msword': 'Word',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': 'Word',
    'application/vnd.ms-excel': 'Excel',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'Excel',
    'application/vnd.ms-powerpoint': 'PPT',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation': 'PPT',
    'application/zip': 'ZIP'
  }
  return fileType[type.toLowerCase()] || '未知类型'
}

const modeSelect = ref(getSetting().mode)

const updateMode = (mode: string) => {
    // 更新组件内部的设置和绑定的变量
    modeSelect.value = mode

    let settingsUpdated = {
      mode: modeSelect.value,
    }
    console.log(settingsUpdated)
    updateSetting(settingsUpdated)
}

const emits = defineEmits<{ (e: 'enterPressed', value: MultiInputValue): void; (e: 'inputValue', value: MultiInputValue): void }>();
const inputValue = ref('')

// not use
// const emits = defineEmits<{ (e: 'getFileList', value: any[]): void }>()
// const getFileList = (val: any) => fileList.value = val

watch([inputValue, fileList], async (val) => {
  console.log(val)
  if(val[1]) {
    const fileMetas: FileMeta[] = await Promise.all(
    val[1].map((file: any) =>
      getFileMeta(file.raw).then((meta) => ({
        fileName: meta.fileName,
        fileType: meta.fileType,
        fileSize: meta.fileSize,
        fileContent: meta.fileContent,
      }))
    )
  );
  const resMap: MultiInputValue = {
    input: val[0],
    fileList: fileMetas
  }
  emits('inputValue', resMap)
  }else {
    const resMap: MultiInputValue = {
    input: val[0],
    fileList: []
  }
  emits('inputValue', resMap)
  }

})

const onEnterPress = async () => {
  console.log('fileList', fileList)
  if(fileList.value) {
    const fileMetas: FileMeta[] = await Promise.all(
      fileList.value.map((file: any) =>
      getFileMeta(file.raw).then((meta) => ({
        fileName: meta.fileName,
        fileType: meta.fileType,
        fileSize: meta.fileSize,
        fileContent: meta.fileContent,
      }))
    )
  )
  const resMap: MultiInputValue = {
    input: inputValue.value,
    fileList: fileMetas
  }
  emits('enterPressed', resMap);
  }else {
    const resMap: MultiInputValue = {
    input: inputValue.value,
    fileList: []
  }
  emits('enterPressed', resMap);
  }
}

</script>
<style lang='scss' scoped>
#multi-input {
  display: flex;
}

.prefix-icon {
  margin-right: 0.5rem;
}

.tag {
  margin-right: 0.5rem;
}



.searchTab {
  margin-left: 15px;
  padding: 8px;
  color: hsla(0, 0%, 100%, 0.7);
  font-size: 15px;
}


:deep(.el-icon svg rect) {
  fill: inherit;
}

.el-radio-group {
  font-size: 15px;
}

:deep(.el-radio-button__inner){
  color: hsla(0, 0%, 100%, 0.7) !important;
}

.searchInput {
  flex-grow: 1;
  padding: 5px;
  border: none;
  border-bottom: 1px solid hsla(0, 0%, 100%, 0.1);
  background: transparent;
}

#file-list {
  display: flex;
  align-items: center;

  .el-tag {
    margin: 0 0.25rem;
  }

  .add-file {
    margin-left: 0.25rem;
  }
}

.mode-active {
  font-weight: bold;
}

.point-send-icon:hover {
  cursor: pointer;
}
</style>