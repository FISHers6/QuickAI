<!--
 * @Author: Joe
 * @Date: 2023-03-22 20:54:15
 * @LastEditors: Joe
 * @LastEditTime: 2023-03-23 00:28:10
 * @FilePath: /ChatGPT-PC/src/components/MarkdownView.vue
 * @Description: 
 * 
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
-->
<template lang='pug'>
VMarkdownEditor(v-model="mdContent", locale="en", v-highlightjs)
</template>
<script lang='ts' setup>
import { VMarkdownEditor } from 'vue3-markdown'
import 'vue3-markdown/dist/style.css'

const props = defineProps({
  response: {
    type: String as PropType<string>,
    required: true
  },
})

const mdContent = computed(() => props.response)


const handleUpload = (val: any) => {
  console.log(val)
  return val
  // val = val.replace('<pre>', '<pre class="code-block">')
  // return 'tetestsetes'
}
// const editor = useEditor({
//   content: props.response,
//   extensions: [
//     StarterKit,
//   ]
// })

onMounted(() => {
  const element: any = document.querySelector(".markdown-body") // 替换成你需要滚动的元素的ID
  element.addEventListener("DOMSubtreeModified", () => {
    element.scrollTop = element.scrollHeight;
  });
})

</script>

<style lang='scss' scoped>
.vmd-box {
  position: absolute;
  top: 0;
  left: 0;
  background-color: transparent;
}

:deep(.vmd-body) {
  border: none;
}

:deep(.vmd-body textarea),
:deep(.vmd-toolbar) {
  display: none !important;
}

:deep(.markdown-body.vmd-view) {
  display: block !important;
  background-color: transparent;
  color: var(--color-white-080);

  &::-webkit-scrollbar {
    display: none;
  }

  pre {
    background-color: var(--color-toolbar);
    color: var(--color-white-080) !important;
    font-weight: 600;
  }

  code {
    display: inline;
    border-radius: 2px;
  }
}
</style>