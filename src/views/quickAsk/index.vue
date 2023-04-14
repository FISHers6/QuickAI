<template lang='pug'>
#ChatGPT4
  .yq
    h4 Your question is:
    el-input.textarea(:suffix-icon="loading ? Loading : Promotion" v-model="question" placeholder="Say something & Enter ... " @keydown.enter.stop="invokeEnter")
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
import { askChatGPTV2,  } from '@/hooks/useAPI'
import { askChatGPTCore  } from '@/hooks/useAPI'

import type { GPTParamV2 } from '@/hooks/useAPI'
import type { GPTResponse } from '@/hooks/useAPI'
import { invoke } from '@tauri-apps/api'
import useClipboard from '@/hooks/useClipboard'
import { onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

import { usePromptModeStore } from '@/hooks/store'
import { useSettings } from "@/hooks/useSettings"
import type { SettingsState } from '@/store/modules/settings/helper'
const { question, getSelectedContent } = useClipboard();
const answer = ref('')
const loading = ref(false)
const controller = new AbortController()

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
    message: '开启指令:' + mode,
    type: 'success',
  })
}

const closePromptNotification = (mode: string) => {
  ElMessage({
    message: '关闭指令:' + mode,
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
      controller: controller,
    }

    const callback = (response: GPTResponse) => {
        if(loading.value) {
            loading.value = false
            if(response) {
                answer.value = response.content
            }
        }else {
            answer.value = response.content
        }
    }

    const errorCallback = (error: any) => {
        loading.value = false
        console.log(error)
        answer.value = error as string
        controller.abort()
    }

    await askChatGPTV2(AskGPTParam, callback, errorCallback)
    console.log('ask start end.')
}

const client_request_api = async () => {
  const messages = [
      {role: 'system', content: promptModeStore.getSelectedPrompt},
      {role: 'user', content: question.value}
    ]

    const {getSetting} = useSettings()
    const setting: SettingsState = getSetting()
    const apiKey = setting.apiKey

    if (apiKey && apiKey!=='') {
      try{
        await askChatGPTCore(messages, apiKey, controller, answer)
        loading.value = false
      }catch(error: any){
        loading.value = false
        answer.value = error.message.includes("The user aborted a request")
          ? ""
          : error.message.replace(/(sk-\w{5})\w+/g, "$1")
      }
    }
}

interface QuestionPayload {
  question: string,
  trigger: boolean,
}

const unlisten = listen('change-question-content', async (event) => {
    console.log(event)
    const questionPayload = (event.payload as QuestionPayload)
    const selected: string = questionPayload.question.trim()
    const trigger = questionPayload.trigger
    console.log('selected: ' + selected);
    console.log(trigger)
    if (selected && selected !== '') {
      question.value = selected
        
      if( trigger ) {
        await askTheQuestion()
      }
    }
});

// onMounted(async () => {
//   try {
//     await getSelectedContent();
//     if(question.value != undefined && question.value !== '') {
//       // await askTheQuestion()
//     }
//   } catch (error) {
//     console.error(error);
//   }
// });

// watchEffect(() => {
//   if (question.value !== '') {
//     console.log(question.value)
//   }
// });

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