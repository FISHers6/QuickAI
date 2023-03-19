import { ref, Ref } from 'vue'
import type { ParsedEvent, ReconnectInterval } from "eventsource-parser"
import { createParser } from "eventsource-parser"

const baseURL = 'api.openai.com'

interface GPTMessage {
    role: string
    content: string
}

async function askChatGPT(question: string, prompts: string, apiKey: string) {
    const answer = ref('')
    const loading = ref(true)

    console.log(123)

    const messages: GPTMessage[] = [
        {role: 'system', content: prompts},
        {role: 'user', content: question}
    ]

    try{
        let response = await askChatGPTCore(messages, apiKey)
        loading.value = false
        answer.value = response
    }catch(error: any){
        loading.value = false
        answer.value = error.message.includes("The user aborted a request")
          ? ""
          : error.message.replace(/(sk-\w{5})\w+/g, "$1")
    }

    return {
        answer,
        loading,
    }
}

async function askChatGPTCore(messages: GPTMessage[], apiKey: string) {
    let response = await askChatGPTAPI(messages, apiKey)

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
    let result = ''

    while (!done) {
        const { value, done: readerDone } = await reader.read()
        if (value) {
            let char = decoder.decode(value)
            if (char === "\n" && result.endsWith("\n")) {
                continue
            }
            if (char) {
                result = result + char
            }
        }
        done = readerDone
    }
    console.log(result)
    return result
}

async function askChatGPTAPI(messages: GPTMessage[], apiKey: string) {
    const encoder = new TextEncoder()
    const decoder = new TextDecoder()

    const body = JSON.stringify({
        model: "gpt-3.5-turbo",
        messages,
        stream: true
    })
    console.log(body)

    const rawRes = await fetch(`https://${baseURL}/v1/chat/completions`, {
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${apiKey}`
      },
      method: "POST",
      body: JSON.stringify({
        model: "gpt-3.5-turbo",
        messages,
        stream: true
      })
    }).catch(err => {
      return new Response(
        JSON.stringify({
          error: {
            message: err.message
          }
        }),
        { status: 500 }
      )
    })

    console.log(rawRes)

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
        for await (const chunk of rawRes.body.pipeThrough(new TextDecoderStream()).pipeThrough(parser)) {
          controller.enqueue(chunk)
        }
      }
    })

    return new Response(stream) 
}

export default askChatGPT