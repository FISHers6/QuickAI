<template lang='pug'>
#ChatGPT4
  .yq
    h4 Your question is:
    el-input.textarea(v-model="question" type='textarea' placeholder="Say something & Enter ... " @keydown.enter="askTheQuestion")
    .prompts
      el-tag(v-for="tag in promptsTags", effect="plain", :key="tag", closable, :disable-transitions="false", @close="handleClose(tag)") {{ tag }}
      el-input.add-tag(v-if="inputVisible", ref="InputRef", v-model="inputValue", size="small" @keyup.enter="handleInputConfirm", @blur="handleInputConfirm")
      el-button(v-else class="button-new-tag ml-1" size="small" @click="showInput") + New Tag
  .chats-says
    h4 ChatGPT says:
    Greet
</template>
<script lang='ts' setup>
import Greet from '@/components/Greet.vue'
import type { ElInput } from 'element-plus'

const question = ref('')
const answer = ref('')

const inputValue = ref('')
const promptsTags: any = ref([])
const inputVisible = ref(false)
const InputRef = ref<InstanceType<typeof ElInput>>()

const handleClose = (tag: string) => promptsTags.value.splice(promptsTags.value.indexOf(tag), 1)

const showInput = () => {
  inputVisible.value = true
  nextTick(() => InputRef.value!.input!.focus())
}

const handleInputConfirm = () => {
  if (inputValue.value) promptsTags.value.push(inputValue.value)
  inputVisible.value = false
  inputValue.value = ''
}

const askTheQuestion = () => {
  alert('Ask The Question')
}
</script>
<style lang='scss' scoped>
#ChatGPT4 {
  color: var(--color-white-100);

  &>* {
    margin-bottom: 1.628rem;
  }

  .yq,
  .chats-says {
    h4 {
      margin-bottom: 1rem;
    }
  }

  .prompts {
    display: flex;
    flex-wrap: wrap;
    margin-top: 0.5rem;

    .el-tag {
      background-color: #915eff34;
    }

    .el-input.add-tag {
      width: 6rem !important;
      height: 24px;
    }

    &>* {
      margin-right: 0.5rem;
      margin-bottom: 0.5rem;

      &:last-child {
        margin-right: 0;
      }
    }

    .el-button {
      border-radius: var(--el-border-radius-base) !important;
      border-color: var(--el-button-text-color);
    }

    .el-button:focus,
    .el-button:hover {
      color: var(--el-color-primary);
      border-color: var(--el-color-primary);
      background-color: #915eff34;
      outline: 0;
    }
  }
}
</style>