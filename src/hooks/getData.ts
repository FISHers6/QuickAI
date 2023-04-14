import base from '@/hooks/base'
import { AI_HEAD_IMG_URL } from '@/hooks/mutation-types'
import { generateUUID } from "@/utils/util"
let axios = base.axios
let baseUrl = base.baseUrl

// 根据提示创建图像
export const createImage = (params, token) => {
  console.log(params)
  return axios({
    method: 'post',
    baseURL: `${baseUrl}/v1/images/generations`,
    headers: {
      'Authorization': 'Bearer ' + token,
      'Content-Type': 'application/json'
    },
    data: params
  }).then(res => {
    return res.data.data;
  })
}

// 根据提示词编辑图像
export const createImageEdit = (formData, token) => {
  return axios({
    method: 'post',
    baseURL: `${baseUrl}/v1/images/edits`,
    headers: {
      'Authorization': 'Bearer ' + token,
      'Content-Type': 'multipart/form-data'
    },
    data: formData
  }).then(res => {
    return res.data.data;
  })
}

// 根据创建图像变体
export const createImageVariations = (formData, token) => {
  return axios({
    method: 'post',
    baseURL: `${baseUrl}/v1/images/variations`,
    headers: {
      'Authorization': 'Bearer ' + token,
      'Content-Type': 'multipart/form-data'
    },
    data: formData
  }).then(res => {
    return res.data.data;
  })
}