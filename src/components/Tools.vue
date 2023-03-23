<template lang='pug'>
#Tools 
  el-button(type="primary") Edit Prompts
  el-popover(popper-class="tools-list")
    template(#reference)
      el-icon.tool-more(color="var(--el-color-primary)" size="24")
        MoreFilled
    .tools-item
      span Dark
      el-switch(v-model="mode" @change="changeMode" size="small")
  //- el-button.more(:icon="MoreFilled" type="primary" link)
</template>
<script lang='ts' setup>
import { MoreFilled } from '@element-plus/icons-vue'
import { setLink, removeLink } from '@/utils'

const mode = ref(false)

const changeMode = (val: boolean) => {
  document.documentElement.classList[ val ? 'add' : 'remove' ]('theme-dark')
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
</style>