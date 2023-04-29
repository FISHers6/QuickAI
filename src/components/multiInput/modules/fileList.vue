<template lang='pug'>
#file-list
  el-tag(v-for="item in fileList" :key="item" effect="plain"  closable @close="handleClose(item)") {{ handleType(item.raw.type) }} | {{ item.name }}
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
import { remove, cloneDeep } from 'lodash'

const emits = defineEmits<{ (e: 'getFileList', value: any[]): void }>()

const fileList: any = ref([])

const handleClose = (item: any) => fileList.value = remove(fileList.value, (val: any) => val.uid !== item.uid)

const handleType = (type: string): string => {
  const fileType: any = {
    'application/pdf': 'PDF',
    'application/msword': 'Word',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': 'Word',
    'application/vnd.ms-excel': 'Excel',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'Excel',
    'application/vnd.ms-powerpoint': 'PPT',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation': 'PPT',
    'application/zip': 'ZIP'
  }
  return fileType[type.toLowerCase()] || '未知类型';
}


watch(fileList, (val: any) => emits('getFileList', cloneDeep(val)))

</script>
<style lang='scss' scoped>
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
</style>