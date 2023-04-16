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
    span.add-file 添加
</template>
<script lang='ts' setup>
import { remove, cloneDeep } from 'lodash'

const emits = defineEmits<{ (e: 'getFileList', value: any[]): void }>()

const fileType: any = {
  text: '文本',
  image: '图片',
  unknown: '未知'
}

const fileList: any = ref([])

const handleClose = (item: any) => fileList.value = remove(fileList.value, (val: any) => val.uid !== item.uid)

const handleType = (type: any) => fileType?.[type?.split('/')[0]] || fileType.unknown

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