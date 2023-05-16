
<template>
  <RouterView />
</template>

<script setup lang="ts">
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api'
import { askChatGPTCore, askChatGPTIntegratorAPI } from '@/hooks/useAPI'
import type { GPTParamV2 } from '@/hooks/useAPI'
import type { GPTResponse } from '@/hooks/useAPI'
import { useSettings } from './hooks/useSettings'

interface TriggerChatAPI {
  question: string,
  prompt: string,
}

const unlisten = listen('trigger-send-chat-api', async (event) => {
    console.log(event)
    const payload = event.payload as TriggerChatAPI
    let question = payload.question.trim()
    if (question && question.length >= 2) {
      send_chat_api(question, payload.prompt)
    }
});

const send_chat_api =  async (question: string, prompts: string) => {
    const controller = new AbortController()


    const {updateSetting, getSetting} = useSettings()
    const setting = getSetting()
  
    const param: GPTParamV2 = {
        question: question,
        prompts: prompts,
        controller: controller,
        conversationID:  setting.conversationRequest?.conversationId,
        parentMessageId: setting.conversationRequest?.parentMessageId
    }
    
    const callback = (response: GPTResponse) => {
          console.log(response)
          if(response) {
              try {
                  invoke('run_auto_input', { payload: { 'response':  response.content } })
              }catch(error: any) {
                  controller.abort()
              }
              console.log(response.content)
          }
    }

    const errorCallback = (error: any) => {
        console.log(error)
    }
    await askChatGPTIntegratorAPI(param, controller, callback, errorCallback)
}
</script>

<style lang="scss">
@import url(./assets/font/iconfont.css);
.iconfont {
  font-family: "iconfont" !important;
  font-style: normal;
  font-size: 25px;
  vertical-align: middle;
  color: rgb(117,120,137);
  transition: .3s;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}
</style>