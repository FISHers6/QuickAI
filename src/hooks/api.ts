import { Ref } from 'vue'
import type { ParsedEvent, ReconnectInterval } from "eventsource-parser"
import { createParser } from "eventsource-parser"
import { useSettings } from "@/hooks/useSettings"

import {Chat} from '@/typings/chat'
import { fetchChatAPIProcess } from '@/api'

import type { SettingsState } from '@/store/modules/settings/helper'

const baseURL = 'api.openai.com'

interface GPTMessage {
    role: string,
    content: string
}

export interface GPTParam {
  question: string, 
  prompts: string, 
  apiKey: string,
}

export interface GPTParamV2 {
  question: string, 
  prompts?: string, // not use
  controller: AbortController,
}

export async function askChatGPTV2(param: GPTParamV2, callback: Function, errorCallback: Function) {
  // 判断空请求
  let question = param.question.trim()
  let prompts: string = param.prompts ? param.prompts.trim() : ''
  if (prompts.length === 0 && (question === '\n' || question.length === 0)) {
    return
  }

// client fetch request
//   const _messages: GPTMessage[] = [
//     {role: 'system', content: prompts},
//     {role: 'user', content: question}
// ]

// const concatenatedContent = messages.reduce((accumulator, current) => {
//   if (current.role === 'user') {
//     return accumulator + current.content;
//   } else {
//     return accumulator;
//   }
// }, ''); 

// if(param.apiKey || param.apiKey!=='') {
//   try{
//     await askChatGPTCore(messages, param.apiKey, controller, result)
//     loading.value = false
// }catch(error: any){
//     loading.value = false
//     result.value = error.message.includes("The user aborted a request")
//       ? ""
//       : error.message.replace(/(sk-\w{5})\w+/g, "$1")
// }
// }

  const {updateSetting, getSetting} = useSettings()
  const setting: SettingsState = getSetting()
  const apiKey = setting.apiKey
  const useChatContext = setting.useChatContext

  let options =	useChatContext ? {
    conversationId: setting.conversationRequest?.conversationId,
    parentMessageId: setting.conversationRequest?.parentMessageId
  } : {
      conversationId: '',
      parentMessageId: ''
  }

  if(apiKey.trim().length === 0) {
    question = prompts.length === 0 ? question : prompts + '.' + question
  }
  
  try {
    const {newConversationId, newParentMessageId} = await fetchChatAPIOnceV2(question, prompts, apiKey, param.controller, options, callback, errorCallback)
    if(useChatContext && (newConversationId !== '' || newParentMessageId !== '')) {
        updateSetting({
          systemMessage: setting.systemMessage,
          language: setting.language,
          apiKey: setting.apiKey,
          proxy: setting.proxy,
          isDarkMode: setting.isDarkMode,
          useChatContext: setting.useChatContext,
          conversationRequest: {
            conversationId: newConversationId,
            parentMessageId: newParentMessageId
          }
        })
    }
  }catch(error: any) {
    console.log(error)
  }
}

// 文本对话 检查指令/image 生成图片
async function fetchChatAPIOnceV2(question: string, prompt: string, apiKey: string, controller: AbortController, options: Chat.ConversationRequest, callback: Function, errorCallback: Function){
  let newConversationId: string = options.conversationId ? options.conversationId : ''
  let newParentMessageId: string = options.parentMessageId ? options.parentMessageId : ''
  await fetchChatAPIProcess<Chat.ConversationResponse>({
    question: question,
    prompt: prompt,
    options,
    apiKey: apiKey,
    signal: controller.signal,
    onDownloadProgress: ({ event }) => {
      const xhr = event.target
      const { responseText } = xhr
      // Always process the final line
      const lastIndex = responseText.lastIndexOf('\n', responseText.length - 2)
      let chunk = responseText
      if (lastIndex !== -1)
        chunk = responseText.substring(lastIndex)
      try {
        const data = JSON.parse(chunk)
        newConversationId = data.conversationId
        newParentMessageId = data.parentMessageId
        callback(data.text)
      }catch (error) {
        console.log(error)
        errorCallback(error)
      }
    },
  })
  return {newConversationId, newParentMessageId}
}

// App组件调用方来传参 如何不用Ref做参数也能做到传进一个响应式对象, 目前的实现是把Result和loading都写进来了
async function askChatGPTCore(messages: GPTMessage[], apiKey: string, controller: AbortController, result: Ref<String>) {  
  let response = await askChatGPTAPI(messages, apiKey, controller)
    if (!response.ok) {
        const res = await response.json()
        throw new Error(res.error.message)
    }
    const data = response.body
    if (!data) {
        throw new Error("没有返回数据")
    }
    const reader = data.getReader()
    const decoder = new TextDecoder("utf-8")
    let done = false

    while (!done) {
        const { value, done: readerDone } = await reader.read()
        if (value) {
            let char = decoder.decode(value)
            if (char === "\n" && result.value.endsWith("\n")) {
                continue
            }
            if (char) {
                result.value = result.value + char
            }
        }
        done = readerDone
    }
}

async function askChatGPTAPI(messages: GPTMessage[], apiKey: string, controller: AbortController) {
    const encoder = new TextEncoder()
    const decoder = new TextDecoder()

    if (!apiKey) {
      throw new Error("请填写OpenAI API key")
    }

    let fut = fetch(`https://${baseURL}/v1/chat/completions`, {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${apiKey}`
      },
      method: "POST",
      body: JSON.stringify({
        model: "gpt-3.5-turbo",
        messages,
        stream: true
      }),
      signal: controller.signal
    })

    const rawRes = await fut.catch(err => {
      return new Response(
        JSON.stringify({
          error: {
            message: err.message
          }
        }),
        { status: 500 }
      )
    })

    if (!rawRes.ok) {
      return new Response(rawRes.body, {
        status: rawRes.status,
        statusText: rawRes.statusText
      })
    }

    const stream = new ReadableStream({
      async start(controller) {
        const streamParser = (event: ParsedEvent | ReconnectInterval) => {
          if (event.type === "event") {
            const data = event.data
            if (data === "[DONE]") {
              controller.close()
              return
            }
            try {
              const json = JSON.parse(data)
              const text = json.choices[0].delta?.content
              const queue = encoder.encode(text)
              controller.enqueue(queue)
            } catch (e) {
              controller.error(e)
            }
          }
        }
        const parser = createParser(streamParser)

        // chrome not support async iterator, so manually read iterator
        async function readAllChunks(readableStream: any) {
          const reader = readableStream.getReader();
          
          let done, value;
          while (!done) {
            ({ value, done } = await reader.read());
            if (done) {
              return;
            }
            parser.feed(decoder.decode(value))
          }
        }

        await readAllChunks(rawRes.body)
      }
    })

    return new Response(stream) 
}