import { Ref } from 'vue'
import type { ParsedEvent, ReconnectInterval } from "eventsource-parser"
import { createParser } from "eventsource-parser"
import { useSettings } from "@/hooks/useSettings"

import {Chat} from '@/typings/chat'

import type { SettingsState } from '@/store/modules/settings/helper'


import type { AxiosProgressEvent, GenericAbortSignal } from 'axios'
import { post } from '@/utils/request'
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

export interface GPTResponse {
  content: string,
  newConversationId: string,
  newParentMessageId: string,
}

export async function askChatGPTV2(param: GPTParamV2, callback: Function, errorCallback: Function) {
  let question = param.question.trim()
  let prompts: string = param.prompts ? param.prompts.trim() : ''
  if (prompts.length === 0 && (question === '\n' || question.length === 0)) {
    return
  }

  const {updateSetting, getSetting} = useSettings()
  const setting: SettingsState = getSetting()
  const apiKey = setting.apiKey
  const useChatContext = setting.useChatContext
  const userProxy = setting.proxy

  let options =	useChatContext ? {
    conversationId: setting.conversationRequest?.conversationId,
    parentMessageId: setting.conversationRequest?.parentMessageId
  } : {
      conversationId: '',
      parentMessageId: ''
  }

  if(apiKey.trim().length === 0) {
    question = prompts.trim().length === 0 ? question : prompts.trim() + '.' + question
  }

  try {
    const {newConversationId, newParentMessageId} = await fetchChatAPIOnceV2(question, prompts, apiKey, userProxy, param.controller, options, callback, errorCallback)
    if(useChatContext && newConversationId && newParentMessageId && (newConversationId !== '' || newParentMessageId !== '')) {
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
async function fetchChatAPIOnceV2(question: string, prompt: string, apiKey: string, userProxy: string, controller: AbortController, options: Chat.ConversationRequest, callback: Function, errorCallback: Function){
  let newConversationId: string = options.conversationId ? options.conversationId : ''
  let newParentMessageId: string = options.parentMessageId ? options.parentMessageId : ''
  await fetchChatAPIProcess<Chat.ConversationResponse>({
    question: question,
    prompt: prompt,
    options,
    apiKey: apiKey,
    userProxy: userProxy,
    signal: controller.signal,
    onDownloadProgress: ({ event }) => {
      const xhr = event.target
      const { responseText } = xhr
      // Always process the final line
      const lastIndex = responseText.lastIndexOf('\n', responseText.length - 2)
      let chunk = responseText
      if (lastIndex !== -1)
        chunk = responseText.substring(lastIndex)
      console.log(chunk)
      try {
        const data = JSON.parse(chunk)
        if(data.status === "Fail" || !data.parentMessageId || data.parentMessageId === '') {
          errorCallback(data.message)
          return
        }
        newConversationId = data.id
        newParentMessageId = data.parentMessageId
        let content = data.text
        console.log('content: ', content)
        let response: GPTResponse =  {
          content: content,
          newConversationId: newConversationId, 
          newParentMessageId: newParentMessageId,
        }
        callback(response)
      }catch (error) {
        console.log(error)
        errorCallback(error)
      }
    },
  })
  return {newConversationId, newParentMessageId}
}

// App组件调用方来传参 如何不用Ref做参数也能做到传进一个响应式对象, 目前的实现是把Result和loading都写进来了
  export async function askChatGPTCore(messages: GPTMessage[], apiKey: string, controller: AbortController, result: Ref<String>) {  
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


// api request methods

// import { useSettingStore } from '@/store'

// 根据提示创建图像
// export const createImage = (params, token) => {
//   console.log(params)
//   return axios({
//     method: 'post',
//     baseURL: `${baseUrl}/v1/images/generations`,
//     headers: {
//       'Authorization': 'Bearer ' + token,
//       'Content-Type': 'application/json'
//     },
//     data: params
//   }).then(res => {
//     return res.data.data;
//   })
// }

// 根据提示词编辑图像
// export const createImageEdit = (formData, token) => {
//   return axios({
//     method: 'post',
//     baseURL: `${baseUrl}/v1/images/edits`,
//     headers: {
//       'Authorization': 'Bearer ' + token,
//       'Content-Type': 'multipart/form-data'
//     },
//     data: formData
//   }).then(res => {
//     return res.data.data;
//   })
// }

// 根据创建图像变体
// export const createImageVariations = (formData, token) => {
//   return axios({
//     method: 'post',
//     baseURL: `${baseUrl}/v1/images/variations`,
//     headers: {
//       'Authorization': 'Bearer ' + token,
//       'Content-Type': 'multipart/form-data'
//     },
//     data: formData
//   }).then(res => {
//     return res.data.data;
//   })
// }

export function createImageVariations<T = any>(formData: FormData) {
  return post<T>({
    url: '/createImageVariations',
    data: { formData },
  })
}

export function createImageEdit<T = any>(formData: FormData) {
  return post<T>({
    url: '/createImageEdit',
    data: { formData },
  })
}


interface imageParams {
  prompt: string,
  n: number,
  size: string,
}

export function createImage<T = any>(params: imageParams, token: string) {
  return post<T>({
    url: '/createImage',
    data: { 
      prompt: params.prompt,
      n: params.n,
      size: params.size,
      token: token,
    },
  })
}

export function fetchChatAPI<T = any>(
  prompt: string,
  options?: { conversationId?: string; parentMessageId?: string },
  signal?: GenericAbortSignal,
) {
  return post<T>({
    url: '/chat',
    data: { prompt, options },
    signal,
  })
}

export function fetchChatConfig<T = any>() {
  return post<T>({
    url: '/config',
  })
}

export function fetchChatAPIProcess<T = any>(
  params: {
    question: string,
    prompt: string
    options?: { conversationId?: string; parentMessageId?: string }
    apiKey?: string
    userProxy?: string
    signal?: GenericAbortSignal
    onDownloadProgress?: (progressEvent: AxiosProgressEvent) => void 
  },
) {

  return post<T>({
    url: '/chat-process',
    data: { prompt: params.question, options: params.options, systemMessage: params.prompt, apiKey: params.apiKey, userProxy: params.userProxy},
    signal: params.signal,
    onDownloadProgress: params.onDownloadProgress,
  })
}

export function fetchSession<T>() {
  return post<T>({
    url: '/session',
  })
}

export function fetchVerify<T>(token: string) {
  return post<T>({
    url: '/verify',
    data: { token },
  })
}
