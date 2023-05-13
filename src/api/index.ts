import type { AxiosProgressEvent, GenericAbortSignal } from 'axios'
import { post } from '@/utils/request'
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
