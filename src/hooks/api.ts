import { Ref } from 'vue'
import type { ParsedEvent, ReconnectInterval } from "eventsource-parser"
import { createParser } from "eventsource-parser"

import {Chat} from '@/typings/chat'
import { fetchChatAPIProcess } from '@/api'

const baseURL = 'api.openai.com'

interface GPTMessage {
    role: string,
    content: string
}

interface GPTParam {
  question: string, 
  prompts: string, 
  apiKey: string,
}
async function askChatGPT(param: GPTParam, result: Ref<String>, loading: Ref<boolean>) {
  let question = param.question.trim()
  let prompts = param.prompts.trim()
  if (!question || question === '\n' || question.length === 0) {
    return
  }
  
  const controller = new AbortController()
  const messages: GPTMessage[] = [
      {role: 'system', content: prompts},
      {role: 'user', content: question}
  ]

  if(param.apiKey || param.apiKey!=='') {
      try{
        await askChatGPTCore(messages, param.apiKey, controller, result)
        loading.value = false
    }catch(error: any){
        loading.value = false
        result.value = error.message.includes("The user aborted a request")
          ? ""
          : error.message.replace(/(sk-\w{5})\w+/g, "$1")
    }
  }else {
    let options=	{
      conversationId: '',
		  parentMessageId: ''
    }
    const concatenatedContent = messages.reduce((accumulator, current) => {
      if (current.role === 'user') {
        return accumulator + current.content;
      } else {
        return accumulator;
      }
    }, '');
    console.log(concatenatedContent)

    try {
      await fetchChatAPIOnce(concatenatedContent, controller, result, options)
      loading.value = false
    }catch(error: any) {
      loading.value = false
    }
  }
}

    // 文本对话 检查指令/image 生成图片
async function fetchChatAPIOnce(message: string, controller: AbortController, result: Ref<String>, options: any){
  await fetchChatAPIProcess<Chat.ConversationResponse>({
    prompt: message,
    options,
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
        console.log('data is', data)
        result.value = result.value + data.text
      }
      catch (error) {
        console.log(error)
      }
    },
  })
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

export default askChatGPT