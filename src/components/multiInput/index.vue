<template lang='pug'>
#multi-input
  el-input(v-model="inputValue" @dblclick="openUpload")
    template(#prefix)
      el-icon.prefix-icon
        Search
      FileList(@getFileList="getFileList")
</template>
<script lang='ts' setup>
import { Search } from '@element-plus/icons-vue'
import FileList from './modules/fileList.vue'
import { cloneDeep } from 'lodash'

const emits = defineEmits<{ (e: 'inputValue', value: any[]): void }>()

const inputValue = ref('')
const fileList = ref([])

const getFileList = (val: any) => fileList.value = val

watch([inputValue, fileList], (val) => {
  const resMap: any = {
    input: val[0],
    fileList: cloneDeep(val[1])
  }
  emits('inputValue', resMap)
})

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
</style>