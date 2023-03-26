<template lang='pug'>
#ChatGPT4
  .yq
    h4 Your question is:
    el-input.textarea(:suffix-icon="loading ? Loading : Promotion" v-model="question" placeholder="Say something & Enter ... " @keydown.stop="invokeEnter")
  .yq
    .prompts
      el-tag(v-for="tag in promptTags", effect="plain", :key="tag.act", closable, :disable-transitions="false", @close="handleClose(tag)" :class="{ highlight: isSelected(tag) }" @click="handleSelectTag(tag)") {{ tag.act }}
      el-input.add-tag(v-if="inputVisible", ref="InputRef", v-model="inputValue", size="small" @keyup.enter="handleInputConfirm", @blur="handleInputConfirm")
      el-button(v-else class="button-new-tag ml-1" size="small" @click="showInput") + New Tag
  .chats-says
    h4 ChatGPT says:
    Result(:response="answer" :loading="loading")
    
</template>

<script lang='ts' setup>
import Result from '@/components/Result.vue'
import Loading from '@/components/loading.vue'
import type { ElInput } from 'element-plus'
import { ElMessage } from 'element-plus'
import { Edit, Promotion } from '@element-plus/icons-vue'
import askChatGPT from '@/hooks/api'
import GPTParam from '@/hooks/api'
import { invoke } from '@tauri-apps/api'
import useClipboard from '@/hooks/useClipboard'
import { onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

import { usePromptModeStore } from '@/hooks/store'

const { question, getSelectedContent } = useClipboard();
const answer = ref('')
const loading = ref(false)

const inputValue = ref('')
const inputVisible = ref(false)
const InputRef = ref<InstanceType<typeof ElInput>>()

const promptModeStore = usePromptModeStore()
const promptTags = computed(() => promptModeStore.items)

const handleClose = (item: any) => {
  promptModeStore.removeByTitle(item.act)
}

const isSelected = (item: any) => {
    return promptModeStore.selectedPrompt && promptModeStore.selectedPrompt.act === item.act
}

const handleSelectTag = (item: any) => {
    if (isSelected(item)) {
        promptModeStore.clearSelectedPrompt()
        closePromptNotification(item.act)
    }else {
        promptModeStore.setSelectedPrompt(item.act)
        openPromptNotification(item.act)
    }
}

const openPromptNotification = (mode: string) => {
  ElMessage({
    message: 'enable ' + mode,
    type: 'success',
  })
}

const closePromptNotification = (mode: string) => {
  ElMessage({
    message: 'close ' + mode,
    type: 'warning',
  })
}

const handleInputConfirm = () => {
  if (inputValue.value) promptModeStore.add(inputValue.value);
  inputVisible.value = false
  inputValue.value = ''
}

const showInput = () => {
  inputVisible.value = true
  nextTick(() => InputRef.value!.input!.focus())
}

const invokeEnter = async ({ isComposing, key }: KeyboardEvent) => {
  // 解决中文输入法回车时触发的bug
  if (!isComposing && key === 'Enter' && !loading.value) {
    if(question.value !== '') {
      await askTheQuestion()
    }
  }
}

const askTheQuestion = async () => {
  loading.value = true
    console.log('ask starting...')
    answer.value = ''
    let AskGPTParam = {
      question: question.value,
      prompts: promptModeStore.getSelectedPrompt,
      apiKey: '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'
    }
    await askChatGPT(AskGPTParam, answer, loading)
    console.log('ask start end.')
}

const unlisten = listen('change-selected-content', (event) => {
    const selected: string = (event.payload as string).trim();
    if (selected && selected !== '') {
      question.value = selected
    }
});

onMounted(async () => {
  try {
    await getSelectedContent();
    if(question.value != undefined && question.value !== '') {
      // await askTheQuestion()
    }
  } catch (error) {
    console.error(error);
  }
});

watchEffect(() => {
  if (question.value !== '') {
    console.log(111)
    console.log(question.value)
    // await askTheQuestion()
  }
});

</script>

<style lang='scss' scoped>
#ChatGPT4 {
  color: var(--color-white-100);

  &>* {
    margin-bottom: 1.628rem;
    &:last-child {
      margin-bottom: 0;
    }
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
    margin-top: 0.2rem;

    .el-tag {
      background-color: #915eff34;

      &.highlight {
        animation: highlight-color 0.5s ease-in-out;
      }
    }

    .el-tag.highlight {
      background-color: var(--color-primary);
      color: var(--color-white-100);
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

@keyframes highlight-color {
  0% {
    background-color: #915eff34;
  }
  50% {
    background-color: #915eff80;
  }
  100% {
    background-color: #915eff34;
  }
}
</style>