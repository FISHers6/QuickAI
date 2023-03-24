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
#mark-down-view
  .code-header(ref="codeHeaderRef")
    .chdr-left
    .chdr-right
      .chdr-tool-item.copy-code
        el-icon(:size="18")
          CopyDocument
  VMarkdownEditor(v-model="mdContent", locale="en", v-highlightjs)
</template>
<script lang='ts' setup>
import { CopyDocument, Check } from '@element-plus/icons-vue'
import { ElMessageBox } from 'element-plus';
import { cloneDeep } from 'lodash';
import { VMarkdownEditor } from 'vue3-markdown'
import 'vue3-markdown/dist/style.css'

const props = defineProps({
  response: {
    type: String as PropType<string>,
    required: true
  }
})

const codeHeaderRef = ref()
const mdContent = computed(() => props.response)

// 给代码的块增加头部栏，包括一些操作按钮
const addPreHeader = () => {
  const pres = document.querySelectorAll("pre")
  pres?.forEach((pre: any) => {
    const fir_pre = pre.firstChild
    const node = codeHeaderRef.value.cloneNode(true)
    pre.insertBefore(node, fir_pre)
  })
}

// 复制功能
const copyCode = (element: HTMLAnchorElement) => {
  const textarea: any = document.createElement('textarea')
  textarea.value = element.textContent
  document.body.appendChild(textarea)
  textarea.select()
  document.execCommand('copy')
  document.body.removeChild(textarea)
}

onMounted(() => {
  // 监听点击复制按钮事件
  document.addEventListener('click', ({ target }: any) => {
    if (target.className.indexOf('copy-code') > -1) {
      const preBlock = target.parentNode.parentNode.parentNode
      copyCode(preBlock)
    }
  })
  const element: any = document.querySelector(".markdown-body") // 替换成你需要滚动的元素的ID
  element.addEventListener("DOMSubtreeModified", () => {
    element.scrollTop = element.scrollHeight
  })

})

onUpdated(() => {
  addPreHeader()
})

</script>

<style lang='scss' scoped>
#mark-down-view {
  .code-header {
    width: 100%;
    height: 2.25rem;
    background-color: #362f46;
    display: none !important;
    padding: 0 1rem;
    align-items: center;
    justify-content: space-between;

    // .chdr-left
    .chdr-right {
      .copy-code {
        &::after {
          content: 'Copy';
          font-size: 12px;
          margin-left: 0.25rem;
          font-weight: 100;
          color: #ffffff;
        }
      }

      .chdr-tool-item {
        height: 1.6rem;
        border-radius: 4px;
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
        transition: all .2s ease;
        padding: 5px;

        .el-icon {
          color: #ffffff !important;
          pointer-events: none;
        }

        &:hover {
          background-color: var(--color-purple);
        }
      }
    }
  }

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
      background-color: #0d0d0d;
      color: var(--color-white-080) !important;
      font-weight: 600;
      position: relative;
      padding-top: calc(3rem + 16px);

      .code-header {
        display: flex !important;
        position: absolute;
        top: 0;
        left: 0;
      }
    }

    code {
      display: inline;
      border-radius: 2px;
    }
  }
}
</style>