/*
 * @Author: Joe
 * @Date: 2023-03-23 00:16:14
 * @LastEditors: Joe
 * @LastEditTime: 2023-03-23 00:19:19
 * @FilePath: /ChatGPT-PC/src/utils/index.ts
 * @Description: 
 * 
 * Copyright (c) 2023 by ${git_name_email}, All Rights Reserved. 
 */
const setLink = (id: string, href: string) => {
  const link = document.createElement('link');
  link.id = id
  link.setAttribute('rel', 'stylesheet');
  link.setAttribute('type', 'text/css');
  link.setAttribute('href', href);
  document.head.appendChild(link);
}

const removeLink = (id: string) => {
  const child: any = document.querySelector(`#${id}`)
  document.head.removeChild(child);
}

export { setLink, removeLink }