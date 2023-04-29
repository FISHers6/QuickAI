import * as dotenv from 'dotenv'
import 'isomorphic-fetch'
import type { ChatGPTAPIOptions, ChatMessage, SendMessageOptions } from 'chatgpt'
import { ChatGPTAPI, ChatGPTUnofficialProxyAPI } from 'chatgpt'
import { SocksProxyAgent } from 'socks-proxy-agent'
import httpsProxyAgent from 'https-proxy-agent'
import proxy from "https-proxy-agent";
import fetch from 'node-fetch'
import nodeFetch from "node-fetch"
import axios from 'axios'
import { sendResponse } from '../utils'
import { isNotEmptyString } from '../utils/is'
import type { ApiModel, ChatContext, ChatGPTUnofficialProxyAPIOptions, ModelConfig } from '../types'
import type { RequestOptions } from './types'

const { HttpsProxyAgent } = httpsProxyAgent

dotenv.config()

const ErrorCodeMessage: Record<string, string> = {
  401: '[OpenAI] 提供错误的API密钥 | Incorrect API key provided',
  403: '[OpenAI] 服务器拒绝访问，请稍后再试 | Server refused to access, please try again later',
  502: '[OpenAI] 错误的网关 |  Bad Gateway',
  503: '[OpenAI] 服务器繁忙，请稍后再试 | Server is busy, please try again later',
  504: '[OpenAI] 网关超时 | Gateway Time-out',
  500: '[OpenAI] 服务器繁忙，请稍后再试 | Internal Server Error',
}

const timeoutMs: number = !isNaN(+process.env.TIMEOUT_MS) ? +process.env.TIMEOUT_MS : 30 * 1000

let apiModel: ApiModel

if (!isNotEmptyString(process.env.OPENAI_API_KEY) && !isNotEmptyString(process.env.OPENAI_ACCESS_TOKEN))
  throw new Error('Missing OPENAI_API_KEY or OPENAI_ACCESS_TOKEN environment variable')

  
const OPENAI_API_BASE_URL = process.env.OPENAI_API_BASE_URL
const OPENAI_API_PROXY = process.env.API_PROXY
let api: ChatGPTAPI | ChatGPTUnofficialProxyAPI
(async () => {
  // More Info: https://github.com/transitive-bullshit/chatgpt-api

  // 优先使用后端配置的api key
  if (isNotEmptyString(process.env.OPENAI_API_KEY)) {
    const OPENAI_API_BASE_URL = process.env.OPENAI_API_BASE_URL
    const OPENAI_API_MODEL = process.env.OPENAI_API_MODEL
    const model = isNotEmptyString(OPENAI_API_MODEL) ? OPENAI_API_MODEL : 'gpt-3.5-turbo'

    const options: ChatGPTAPIOptions = {
      apiKey: process.env.OPENAI_API_KEY,
      completionParams: { model },
      debug: true,
    }

    // increase max token limit if use gpt-4
    if (model.toLowerCase().includes('gpt-4')) {
      // if use 32k model
      if (model.toLowerCase().includes('32k')) {
        options.maxModelTokens = 32768
        options.maxResponseTokens = 8192
      }
      else {
        options.maxModelTokens = 8192
        options.maxResponseTokens = 2048
      }
    }

    if (isNotEmptyString(OPENAI_API_BASE_URL))
      options.apiBaseUrl = `${OPENAI_API_BASE_URL}/v1`

    setupProxy(options)
    api = new ChatGPTAPI({     
      fetch: isNotEmptyString(OPENAI_API_PROXY) ? (url, options = {}) => {
        const defaultOptions = {
          agent: proxy(OPENAI_API_PROXY),
        };

        const mergedOptions = {
          ...defaultOptions,
          ...options,
        };

      return nodeFetch(url, mergedOptions);
    }: fetch, ...options })
    apiModel = 'ChatGPTAPI'
  }
  else {
    const OPENAI_API_MODEL = process.env.OPENAI_API_MODEL
    const options: ChatGPTUnofficialProxyAPIOptions = {
      accessToken: process.env.OPENAI_ACCESS_TOKEN,
      debug: true,
    }
    if (isNotEmptyString(OPENAI_API_MODEL))
      options.model = OPENAI_API_MODEL

    if (isNotEmptyString(process.env.API_REVERSE_PROXY))
      options.apiReverseProxyUrl = process.env.API_REVERSE_PROXY

    setupProxy(options)

    api = new ChatGPTUnofficialProxyAPI({ ...options })
    apiModel = 'ChatGPTUnofficialProxyAPI'
  }
})()

async function chatReplyProcess(options: RequestOptions) {
  const { message, lastContext, process, systemMessage, apiKey, userProxy } = options

  // add visit url custom in options
  
  try {
    let options: SendMessageOptions = { timeoutMs }

    if (apiModel === 'ChatGPTAPI' || isNotEmptyString(apiKey))  {
      if (isNotEmptyString(systemMessage))
        options.systemMessage = systemMessage
    }

    if (lastContext != null) {
      if (apiModel === 'ChatGPTAPI' || isNotEmptyString(apiKey))
        // need conversion id
        // options.parentMessageId = lastContext.parentMessageId
        options = { ...lastContext }
      else
        options = { ...lastContext }
    }
    let user_proxy_api: ChatGPTAPI | ChatGPTUnofficialProxyAPI
    if (isNotEmptyString(apiKey)) {

      // not support now
      // const model = isNotEmptyString(OPENAI_API_MODEL) ? OPENAI_API_MODEL : 'gpt-3.5-turbo'
      const model = 'gpt-3.5-turbo'
      const options: ChatGPTAPIOptions = {
        apiKey: apiKey,
        completionParams: { model },
        debug: false,
      }
  
      // increase max token limit if use gpt-4
      if (model.toLowerCase().includes('gpt-4')) {
        // if use 32k model
        if (model.toLowerCase().includes('32k')) {
          options.maxModelTokens = 32768
          options.maxResponseTokens = 8192
        }
        else {
          options.maxModelTokens = 8192
          options.maxResponseTokens = 2048
        }
      }
      
      if (isNotEmptyString(OPENAI_API_BASE_URL))
        options.apiBaseUrl = `${OPENAI_API_BASE_URL}/v1`
      if (isNotEmptyString(userProxy))
        options.apiBaseUrl = `${userProxy}/v1`

      console.log('use user api key')
      console.log(options)

      user_proxy_api = new ChatGPTAPI({     
        fetch: isNotEmptyString(OPENAI_API_PROXY) ? (url, options = {}) => {
          const defaultOptions = {
            agent: proxy(OPENAI_API_PROXY),
          };
  
          const mergedOptions = {
            ...defaultOptions,
            ...options,
          };
  
        return nodeFetch(url, mergedOptions);
      }: fetch, ...options })
    }else {
      console.log('use backend api key')
      user_proxy_api = api
    }

    const response = await user_proxy_api.sendMessage(message, {
      ...options,
      onProgress: (partialResponse) => {
        process?.(partialResponse)
      },
    })

    return sendResponse({ type: 'Success', data: response })
  }
  catch (error: any) {
    const code = error.statusCode
    global.console.log(error)
    if (Reflect.has(ErrorCodeMessage, code))
      return sendResponse({ type: 'Fail', message: ErrorCodeMessage[code] })
    return sendResponse({ type: 'Fail', message: error.message ?? 'Please check the back-end console' })
  }
}

async function fetchBalance() {
  const OPENAI_API_KEY = process.env.OPENAI_API_KEY
  const OPENAI_API_BASE_URL = process.env.OPENAI_API_BASE_URL

  if (!isNotEmptyString(OPENAI_API_KEY))
    return Promise.resolve('-')

  const API_BASE_URL = isNotEmptyString(OPENAI_API_BASE_URL)
    ? OPENAI_API_BASE_URL
    : 'https://api.openai.com'

  try {
    const headers = { 'Content-Type': 'application/json', 'Authorization': `Bearer ${OPENAI_API_KEY}` }
    const response = await axios.get(`${API_BASE_URL}/dashboard/billing/credit_grants`, { headers })
    const balance = response.data.total_available ?? 0
    return Promise.resolve(balance.toFixed(3))
  }
  catch {
    return Promise.resolve('-')
  }
}

async function chatConfig() {
  const balance = await fetchBalance()
  const reverseProxy = process.env.API_REVERSE_PROXY ?? '-'
  const httpsProxy = (process.env.HTTPS_PROXY || process.env.ALL_PROXY) ?? '-'
  const socksProxy = (process.env.SOCKS_PROXY_HOST && process.env.SOCKS_PROXY_PORT)
    ? (`${process.env.SOCKS_PROXY_HOST}:${process.env.SOCKS_PROXY_PORT}`)
    : '-'
  return sendResponse<ModelConfig>({
    type: 'Success',
    data: { apiModel, reverseProxy, timeoutMs, socksProxy, httpsProxy, balance },
  })
}

function setupProxy(options: ChatGPTAPIOptions | ChatGPTUnofficialProxyAPIOptions) {
  if (process.env.SOCKS_PROXY_HOST && process.env.SOCKS_PROXY_PORT) {
    const agent = new SocksProxyAgent({
      hostname: process.env.SOCKS_PROXY_HOST,
      port: process.env.SOCKS_PROXY_PORT,
    })
    options.fetch = (url, options) => {
      return fetch(url, { agent, ...options })
    }
  }
  else {
    console.log('set up proxy')
    if (process.env.HTTPS_PROXY || process.env.ALL_PROXY) {
      const httpsProxy = process.env.HTTPS_PROXY || process.env.ALL_PROXY
      console.log(httpsProxy)
      if (httpsProxy) {
        const agent = new HttpsProxyAgent(httpsProxy)
        options.fetch = (url, options) => {
          return fetch(url, { agent, ...options })
        }
      }
    }
  }
}

const baseUrl = 'https://api.openai.com'
interface ImageGenerationParams {
  // TODO: 根据实际需要定义接口参数 
  prompt: string,
  n: number,
  size: string,
}

interface ImageGenerationAPIParams {
  // TODO: 根据实际需要定义接口参数 
  prompt: string,
  n: number,
  size: string,
  token: string
}

const createImage = async (params: ImageGenerationParams, token: string): Promise<any> => {
  const url = `${baseUrl}/v1/images/generations`
  const headers = {
    Authorization: `Bearer ${token}`,
    'Content-Type': 'application/json',
  };
  console.log(url)
  const body = JSON.stringify(params)
  const response = await fetch(url, {
    method: 'POST', headers, body,
  }); 
  console.log(response)
  const data = await response.json()
  return data.data;
};

function currentModel(): ApiModel {
  return apiModel
}

export type { ChatContext, ChatMessage, ImageGenerationParams, ImageGenerationAPIParams}

export { chatReplyProcess, chatConfig, currentModel, createImage }