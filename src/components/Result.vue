<template lang="pug">
.says-area
  .empty-area(v-if="!response")
    el-icon.icon
      EditPen
  MarkdownView(v-else :response="response")
</template>
  
<script lang="ts" setup>
import { Promotion, WarnTriangleFilled, EditPen } from '@element-plus/icons-vue'
import MarkdownView from './MarkdownView.vue';

const props = defineProps({
  response: {
    type: String as PropType<string>,
    required: true
  },
  loading: {
    type: Boolean as PropType<boolean>,
    required: true
  }
})
</script>
  
<style lang="scss" scoped>
.says-area {
  position: relative;
  height: calc(100vh - 25.85rem);
  background-color: var(--el-fill-color-blank);
  border-radius: var(--radius-1);
  padding: 5px 11px;
  box-sizing: border-box;
  color: var(--color-white-080);
  overflow: hidden;

  .empty-area {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .icon {
      font-size: 3rem;
      color: var(--color-white-003);
    }
  }

  .speaking {
    position: relative;
  }
}
</style>
  