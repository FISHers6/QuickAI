<template>
  <div class="chat-window">
    <!-- <div class="other-fun">
      <label @click="snapchat">
        <span class="iconfont icon-snapchat"></span>
      </label>
    </div> -->
    <div class="botoom">
      <div class="chat-content" id="chat-content" ref="scrollRef">
        <div class="chat-wrapper" v-for="(item, index) of dataSources" :key="index">
          <div :class="item.inversion ? 'chat-me': 'chat-friend'">
            <div class="chat-text" v-if="item.messageType == 0">
              <el-row :gutter="20">
                <el-col :span="2">
                </el-col>
                <el-col :span="21"> </el-col>
              </el-row>
              <!-- <MarkdownView :response='item.text'></MarkdownView> -->
              {{ item.text }}
              <!-- <markdown-it-vue :content="item.msg.trim()" /> -->
            </div>
            <div class="chat-img" v-if="item.messageType == 1">
              <img :src="item.imageUrl" alt="表情" v-if="item.imgType == 1" class="emoji-message" />
              <el-image style="max-width: 300px; border-radius: 10px" :src="item.imageUrl" :preview-src-list= "item.imageUrl ? [item.imageUrl] : []" v-else>
              </el-image>
            </div>
            <div class="chat-file" v-if="item.messageType == 2">
              <div class="word-file">
                <FileCard :fileType="item.fileType ? item.fileType : 0" :file="getFile(item.fileMeta)"></FileCard>
              </div>
            </div>
            <div class="info-time">
              <!-- <img :src="item.headImg" alt="" />
              <span>{{ item.name }}</span> -->
              <span>{{ item.dateTime }}</span>
            </div>
          </div>

        </div>
      </div>
      <div class="chatInputs">
        <!--表情-->
        <div class="emoji boxinput" @click="clickEmoji">
          <img src="@/assets/img/emoji/smiling-face.png" alt="" />
        </div>
        <label class="emoji boxinput" for="imgFile"><span class="iconfont icon-tupian"></span></label>
        <label class="emoji boxinput" for="docFile"><span class="iconfont icon-wenjian"></span></label>
        <input v-show="false" type="file" name="" id="imgFile" @change="sendImg" accept="image/*" />
        <input v-show="false" type="file" name="" id="docFile" @change="sendFile" accept="application/*,text/*" />
        <!--emo表情列表-->
        <div class="emoji-content" v-show="showEmoji">
          <Emoji v-show="showEmoji" @sendEmoji="sendEmoji" @closeEmoji="clickEmoji"></Emoji>
        </div>
        <!--输入框-->
        <el-input
          type="textarea"
          id="textareaMsg"
          ref="inputRef"
          v-model="prompt"
          :autosize="{}"
          class="textarea"
          maxlength="2000"
          placeholder="请输入你的问题~"
          style="margin-left: 2%;margin-top: 3px;min-height: 51px;max-height:400px;max-width: 80%;min-width: 45%;  height: auto;" 
        ></el-input>
        <div>
          <div class="send boxinput" @click='handleSubmit'>
            <!-- <img src="@/assets/img/emoji/rocket.png" alt="" /> -->
            <Promotion class="send-button"></Promotion>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
  
<script setup lang='ts'>
import html2canvas from 'html2canvas'
import Emoji from "@/components/Emoji.vue"
import FileCard from "@/components/FileCard.vue"
import MarkdownView from "@/components/MarkdownView.vue"
import { useChat } from '@/hooks/useChat'
import { Promotion } from '@element-plus/icons-vue'
import { useRoute } from 'vue-router'
import { useChatStore, usePromptStore } from '@/store'
import { fetchChatAPIProcess } from '@/api'
import { useScroll } from '@/hooks/useScroll'
import { getFile, getFileMeta} from "@/hooks/useFile"
import FileMeta from "@/hooks/useFile"
import { Chat } from '@/typings/chat'
import { createImageEdit, createImageVariations} from "@/hooks/getData"
import { createImage} from "@/api/index"
import { listen } from '@tauri-apps/api/event';
import { askChatGPTV2 } from '@/hooks/api'
import type { GPTParamV2 } from '@/hooks/api'
import type { GPTResponse } from '@/hooks/api'

const { scrollRef, scrollToBottom, scrollToBottomIfAtBottom } = useScroll()
const openLongReply = true

// 用于存储chat和message
const { addChat, updateChat, updateChatSome, getChatByUuidAndIndex } = useChat()

// 消息发送控制停止
let controller = new AbortController()


// 会话内存状态
const chatStore = useChatStore()

// 会话id
// const route = useRoute()
// const { uuid } = route.params as { uuid: string }
const uuid: number = chatStore.getLatestChatID()
console.log('uuid: ', uuid)

// 消息内存状态
const dataSources = computed(() => {
  let chatMessages = chatStore.getChatByUuid(+uuid)
  console.log('chatMessages')
  console.log(chatMessages)
  return chatMessages
})
// 当前会话ChatGPT发的消息
const conversationList = computed(() => dataSources.value.filter(item => (!item.inversion && !item.error)))

// 当前会话用户发的消息
const userMessageList = computed(() => dataSources.value.filter(item => (item.inversion && !item.error)))

const apiKey = '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'

// 发送的消息
const prompt = ref<string>('')
// 是否加载中
const loading = ref<boolean>(false)
// 输入框?
const inputRef = ref<Ref | null>(null)

// 聊天信息内容
const chatContent = ref<HTMLDivElement | null>(null)

// 预览图
const srcImgList = ref<string[]>([])

function handleSubmit() {
  if(prompt.value === '' || prompt.value.trim() === '') {
    return
  }
  let chatMsg = {
    time: new Date().toLocaleString(),
    msg: prompt.value.trim(), // 文本, 图片源地址
    messageType: 0, //信息类型，0文字，1图片 2文件
  }
  onConversation(chatMsg)
}

onMounted(() => {
  console.log(chatContent.value?.scrollHeight)
  scrollToBottom()
})

const unlisten = listen('change-chat-question-content', async (event) => {
    console.log(event)
    const changed_question = (event.payload as string).trim()
    console.log('trigger question: ' + changed_question);
    if (changed_question &&changed_question !== '' && prompt.value !== changed_question) {
      prompt.value = changed_question
      handleSubmit()
    }
});

// 创建会话
async function onConversation(chatMsg: ChatMsg) {
  let message = chatMsg.msg
  let userMessage = chatMsg.msg
  let dateTime = chatMsg.time
  let messageType = chatMsg.messageType
  let imageUrl = chatMsg.imageUrl
  let fileMeta = chatMsg.fileMeta
  let fileType = chatMsg.fileType
  let imgType = chatMsg.imgType

  if (loading.value)
    return

  controller = new AbortController()

  if (userMessageList.value.length > 0) {
    const lastMessage: Chat.Chat | null | undefined = userMessageList.value[userMessageList.value.length - 1]
    if (lastMessage && lastMessage.messageType !== 0) {
      if(lastMessage.messageType === 2) {
        message += lastMessage.fileMeta?.fileContent
      }
    }
  }

  // 用户发的消息
  addChat(
    +uuid,
    {
      dateTime: dateTime,
      text: userMessage,
      inversion: true,
      error: false,
      conversationOptions: null,
      requestOptions: { prompt: message, options: null },
      messageType: messageType,
      imageUrl: imageUrl,
      fileMeta: fileMeta,
      fileType: fileType,
      imgType: imgType,
    },
  )
  scrollToBottom()

  if(imageUrl || fileMeta) {
    console.log(imageUrl, fileMeta)
    return
  }

  if (messageType == 0 && (!message || message.trim() === ''))
    return

  loading.value = true
  prompt.value = ''

  let options: Chat.ConversationRequest = {}
  const lastContext = conversationList.value[conversationList.value.length - 1]?.conversationOptions

  // if (lastContext && usingContext.value)
  if (lastContext)
    options = { ...lastContext }

  // 机器人消息 假消息先上屏
  addChat(
    +uuid,
    {
      dateTime: new Date().toLocaleString(),
      text: '',
      messageType: 0,
      loading: true,
      inversion: false,
      error: false,
      conversationOptions: null,
      requestOptions: { prompt: message, options: { ...options } },
    },
  )
  scrollToBottom()

  try {
    let lastText = ''

    const genImageAPI = (prompt: string, num: number, size: string) => {
      let params = {
        prompt: prompt,
        n: num,
        size: size
      }
      createImage(params, apiKey).then(data => {
        for (let imgInfo of data) {
          let chatImgMsg = {
            time: new Date().toLocaleString(),
            msg: imgInfo.url,
            messageType: 1, //信息类型，0文字，1图片 2文件
            imgType: 2, //(1表情，2本地图片)
            imageUrl: imgInfo.url
          };
          addChatMessage(chatImgMsg)
          srcImgList.value.push(imgInfo.url)
        }
      })
    }

    /*
    // 文本对话 检查指令/image 生成图片
      const fetchChatAPIOnce = async () => {
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
              updateChat(
                +uuid,
                dataSources.value.length - 1,
                {
                  dateTime: new Date().toLocaleString(),
                  text: lastText + data.text ?? '',
                  inversion: false,
                  messageType: 0,
                  error: false,
                  loading: false,
                  conversationOptions: { conversationId: data.conversationId, parentMessageId: data.id },
                  requestOptions: { prompt: message, options: { ...options } },
                },
              )

              if (openLongReply && data.detail.choices[0].finish_reason === 'length') {
                options.parentMessageId = data.id
                lastText = data.text
                message = ''
                return fetchChatAPIOnce()
              }

              scrollToBottomIfAtBottom()
            }
            catch (error) {
            //
            }
          },
        })
      }
    */
    // 图片: 调用改图api 

    // 文件: 解析成文本, 再调用文本对话
    if(message.startsWith('/image') || message.startsWith('/img') || message.startsWith('/生成图片') || message.startsWith('/图片生成') || message.startsWith('/图片 ')) {
      genImageAPI(message, 1, "240x240")
    }else {
      const param: GPTParamV2 = {
        question: message,
        prompts: '',
        controller: controller,
    }

    const callback = (response: GPTResponse) => {
        if(loading.value) {
          loading.value = false
        }
        console.log(response)
        lastText = response.content
        updateChat(
          +uuid,
          dataSources.value.length - 1,
          {
            dateTime: new Date().toLocaleString(),
            text: response.content,
            inversion: false,
            messageType: 0,
            error: false,
            loading: false,
            conversationOptions: { conversationId: response.newConversationId, parentMessageId: response.newParentMessageId },
            requestOptions: { prompt: message, options: { ...options } },
          },
        )
        options.conversationId = response.newParentMessageId
        options.parentMessageId = response.newParentMessageId
        scrollToBottomIfAtBottom()
    }

      const errorCallback = (error: any) => {
          console.log(error)
          controller.abort()
          loading.value = false
      }

      // 文本对话
      await askChatGPTV2(param, callback, errorCallback)
    }
  }
  catch (error: any) {
    // const errorMessage = error?.message ?? t('common.wrong')
    const errorMessage = error?.message

    if (error.message === 'canceled') {
      updateChatSome(
        +uuid,
        dataSources.value.length - 1,
        {
          loading: false,
        },
      )
      scrollToBottomIfAtBottom()
      return
    }

    const currentChat = getChatByUuidAndIndex(+uuid, dataSources.value.length - 1)

    if (currentChat?.text && currentChat.text !== '') {
      updateChatSome(
        +uuid,
        dataSources.value.length - 1,
        {
          text: `${currentChat.text}\n[${errorMessage}]`,
          error: false,
          loading: false,
        },
      )
      return
    }

    updateChat(
      +uuid,
      dataSources.value.length - 1,
      {
        dateTime: new Date().toLocaleString(),
        text: errorMessage,
        inversion: false,
        error: true,
        loading: false,
        conversationOptions: null,
        requestOptions: { prompt: message, options: { ...options } },
        messageType: 0,
      },
    )
    scrollToBottomIfAtBottom()
  }
  finally {
    loading.value = false
  }
}


const showEmoji = ref(false)

//关闭标签框
const clickEmoji = () =>  {
  showEmoji.value = !showEmoji.value
}

interface ChatMsg {
  time: string,
  msg: string, // 文本
  imageUrl?: string,
  fileMeta?: FileMeta,
  messageType: number, //信息类型，0文字，1图片 2文件 4富文本
  imgType?: number, //(1表情，2本地图片)
  fileType?: number,
}

//发送表情
const sendEmoji = (url) => {
  console.log(url)
  const dateNow = new Date().toLocaleString();
  let chatMsg = {
    // headImg: USER_HEAD_IMG_URL,
    // name: USER_NAME,
    time: dateNow,
    imageUrl: url,
    msg: '',
    messageType: 1, //信息类型，0文字，1图片
    imgType: 1, //(1表情，2本地图片)
    // uid: "jcm",
  };
  console.log('emoji:', chatMsg);
  addChatMessage(chatMsg);
  clickEmoji();
}

//发送信息
const addChatMessage = (chatMsg: ChatMsg) => {
  let message = chatMsg.msg
  let userMessage = chatMsg.msg
  let dateTime = chatMsg.time
  let messageType = chatMsg.messageType
  let imageUrl = chatMsg.imageUrl
  let fileMeta = chatMsg.fileMeta
  let fileType = chatMsg.fileType
  let imgType = chatMsg.imgType

  if (loading.value)
    return
  // 用户发的消息
  addChat(
    +uuid,
    {
      dateTime: dateTime,
      text: userMessage,
      inversion: true,
      error: false,
      conversationOptions: null,
      requestOptions: { prompt: message, options: null },
      messageType: messageType,
      imageUrl: imageUrl,
      fileMeta: fileMeta,
      fileType: fileType,
      imgType: imgType,
    },
  )
  scrollToBottom()
}

//截图
const snapchat = () => {
  const contentEle = document.querySelector('#chat-content')
  const options = {
    backgroundColor: "rgb(39, 42, 55)" // 设置截图背景颜色
  };
  if(contentEle) {
    html2canvas(contentEle, options).then(canvas => {
      canvas.toBlob(blob => {
        const url = URL.createObjectURL(blob);
        const link = document.createElement('a');
        link.download = 'screenshot.png';
        link.href = url;
        link.click();
        URL.revokeObjectURL(url);
      });
    })
  }
}

//发送文件
const sendFile = async (e) => {
    const dateNow = new Date().toLocaleString();

    let files: File = e.target.files[0]; //文件名
    let chatMsg = {
      // headImg: USER_HEAD_IMG_URL,
      // name: USER_NAME,
      time: dateNow,
      msg: "",
      messageType: 2, //信息类型，0文字，1图片, 2文件
      fileType: 4, //(1word，2excel，3ppt，4pdf，5zpi, 6txt)
      fileMeta: await getFileMeta(files),
      // uid: "jcm",
    };
    // todo: 文件解析
    console.log(files.type)

    if (files) {
      switch (files.type) {
        case "application/msword":
        case "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
          chatMsg.fileType = 1;
          break;
        case "application/vnd.ms-excel":
        case "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet":
          chatMsg.fileType = 2;
          break;
        case "application/vnd.ms-powerpoint":
        case "application/vnd.openxmlformats-officedocument.presentationml.presentation":
          chatMsg.fileType = 3;
          break;
        case "application/pdf":
          chatMsg.fileType = 4;
          break;
        case "application/zip":
        case "application/x-zip-compressed":
          chatMsg.fileType = 5;
          break;
        case "text/plain":
          chatMsg.fileType = 6;
          break;
        default:
          chatMsg.fileType = 0;
      }
      addChatMessage(chatMsg);
      e.target.files = null;
    }
}

const loadImageAsync =  (filesdata) => {
  return new Promise( function( resolve, reject){
    var reader = new FileReader();
    reader.readAsDataURL(filesdata); // 读出 base64 
    reader.onloadend = function(){
      resolve(reader);
    };
    reader.onerror = function () {
      reject(new Error("could not load image at ") );
    };            
});
}

//发送本地图片
const sendImg = (e) => {
  // acqStatus = false
  //获取文件
  const file = e.target.files[0];

  console.log('chatMsg image file: ', file);

  // 验证文件类型是否为PNG格式
  if (file.type !== "image/png") {
    // this.$message({
    //   message: "请上传一个有效的PNG文件~",
    //   type: "warning",
    // });
    // this.acqStatus = true
    return;
  }

  // 验证文件大小是否小于4MB
  if (file.size > 4 * 1024 * 1024) {
    // this.$message({
    //   message: "请上传一个小于4MB的文件~",
    //   type: "warning",
    // });
    // this.acqStatus = true
    return;
  }

  // if (this.settingInfo.openChangePicture) {
  //   this.updateImage = file
  //   this.$message({
  //     message: "图片上传完成啦，请给我提示进行编辑~",
  //     type: "info",
  //   });
  //   e.target.files = null;
  //   this.acqStatus = true
  //   return
  // }
  
  // 通过验证后，上传文件
  const formData = new FormData();
  formData.append("image", file);
  formData.append("n", '1');
  // formData.append("size", this.settingInfo.size);

  let chatMsg = {
    // headImg: USER_HEAD_IMG_URL,
    // name: USER_NAME,
    time: new Date().toLocaleString(),
    msg: "",
    messageType: 1, //信息类型，0文字，1图片 2文件
    imgType: 2, //(1表情，2本地图片)
    imageUrl: "",
  };

  if (!e || !window.FileReader) return;
  let reader = new FileReader();
  reader.readAsDataURL(file);
  reader.onloadend = function (e) {
    if(e.target && e.target.result) {
      chatMsg.imageUrl = e.target.result.toString();
      srcImgList.value.push(chatMsg.imageUrl);
      addChatMessage(chatMsg);
      createImageVariations(formData, apiKey).then(data => {
      for (var imgInfo of data) {
        let imgResMsg = {
          time: new Date().toLocaleString(),
          msg: imgInfo.url,
          messageType: 1, //信息类型，0文字，1图片
          imgType: 2, //(1表情，2本地图片)
        };
        addChatMessage(imgResMsg);
        srcImgList.value.push(imgInfo.url);
      }
    })
    }
  };
  e.target.files = null;
}

</script>

<style lang="scss" scoped>

:deep(.el-textarea__inner){
  background-color: rgb(48 51 59);
    border-radius: 15px;
    border: 2px solid rgb(49 50 51);
    /* padding: 10px; */
    box-sizing: border-box;
    transition: 0.2s;
    font-size: 20px;
    color: #fff;
    font-weight: 100;
    /* margin: 0 20px; */
    width: 98%;
    height: 100%;

}

.botoom {
  background-color: rgb(25 26 29)
}

.emoji-message {
  width: 50px; 
  height: 50px
}

textarea::-webkit-scrollbar {
  width: 3px;
  /* 设置滚动条宽度 */
}

textarea::-webkit-scrollbar-thumb {
  background-color: rgb(66, 70, 86);
  /* 设置滚动条滑块的背景色 */
  border-radius: 50%;
  /* 设置滑块的圆角 */
}

.spinner {
  width: 50px;
  height: 50px;
  animation: spin 1s infinite linear;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }

  100% {
    transform: rotate(360deg);
  }
}

.chat-window {
  width: 90%;
  height: 100%;
  margin-left: 20px;
  position: relative;

  .top {
    &::after {
      content: "";
      display: block;
      clear: both;
    }

    .head-pic {
      float: left;
    }

    .info-detail {
      float: left;
      margin: 5px 20px 0;

      .name {
        font-size: 20px;
        font-weight: 600;
        color: #fff;
      }

      .detail {
        color: #9e9e9e;
        font-size: 12px;
        margin-top: 2px;
      }
    }

    .other-fun {
      float: right;
      margin-top: 20px;

      span {
        margin-left: 30px;
        cursor: pointer;
      }

      input {
        display: none;
      }
    }
  }


  .textarea {
    &:focus {
      outline: none;
    }
  }

  .botoom {
    width: 100%;
    height: 75vh;
    background-size: 100% 100%;
    border-radius: 20px;
    padding: 20px;
    box-sizing: border-box;
    position: relative;

    .chat-content {
      width: 100%;
      height: 85%;
      overflow-y: scroll;
      padding: 20px;
      box-sizing: border-box;

      &::-webkit-scrollbar {
        width: 3px;
        /* 设置滚动条宽度 */
      }

      &::-webkit-scrollbar-thumb {
        background-color: rgb(66, 70, 86);
        /* 设置滚动条滑块的背景色 */
        border-radius: 50%;
        /* 设置滑块的圆角 */
      }

      .chat-friend {
        width: 100%;
        float: left;
        margin-bottom: 20px;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        align-items: flex-start;

        .chat-text {
          float: left;
          max-width: 90%;
          padding: 15px;
          border-radius: 20px 20px 20px 5px;
          background-color: #fff;
        }

        .chat-img {
          img {
            max-width: 300px;
            max-height: 200px;
            border-radius: 10px;
          }
        }

        .chat-file {
          img {
            max-width: 300px;
            max-height: 200px;
            border-radius: 10px;
          }
        }

        .info-time {
          margin: 10px 0;
          color: #fff;
          font-size: 14px;
          display: flex;
          justify-content: flex-start;

          img {
            width: 30px;
            height: 30px;
            border-radius: 50%;
            vertical-align: middle;
            margin-right: 10px;
          }

          span {
            line-height: 30px;
          }

          span:last-child {
            color: rgb(101, 104, 115);
            margin-left: 10px;
            vertical-align: middle;
          }
        }
      }

      .chat-me {
        width: 100%;
        float: right;
        margin-bottom: 20px;
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-end;
        align-items: flex-end;

        .chat-text {
          float: right;
          max-width: 90%;
          padding: 15px;
          border-radius: 20px 20px 5px 20px;
          background-color: #95ec69;
          color: #000;
        }

        .chat-img {
          img {
            max-width: 300px;
            max-height: 200px;
            border-radius: 10px;
          }
        }

        .info-time {
          margin: 10px 0;
          color: #fff;
          font-size: 14px;
          display: flex;
          justify-content: flex-end;

          img {
            width: 30px;
            height: 30px;
            border-radius: 50%;
            vertical-align: middle;
            margin-left: 10px;
          }

          span {
            line-height: 30px;
          }

          span:first-child {
            color: rgb(101, 104, 115);
            margin-right: 10px;
            vertical-align: middle;
          }
        }
      }
    }

    .chatInputs {
      width: 90%;
      position: absolute;
      bottom: 0;
      margin: 1% 0% 1% 0%;
      display: flex;

      .boxinput {
        width: 40px;
        height: 40px;
        background-color: rgb(50, 54, 68);
        border-radius: 15px;
        border: 1px solid rgb(80, 85, 103);
        box-shadow: 0px 0px 5px 0px rgb(0, 136, 255);
        position: relative;
        cursor: pointer;
        margin-left: 5px;
        transform: translate(0%, 10%);

        img {
          width: 30px;
          height: 30px;
          position: absolute;
          left: 50%;
          top: 50%;
          transform: translate(-50%, -50%);
        }

        .iconfont {
          position: absolute;
          left: 50%;
          top: 50%;
          transform: translate(-50%, -50%);
        }
        .send-button {
          width: 30px;
          height: 30px;
          position: absolute;
          left: 50%;
          top: 50%;
          transform: translate(-50%, -50%);
          color: #757889
        }
        
      }

      .emoji {
        background-color: rgb(48 51 59);
        border: 0;
        transition: 0.3s;
        box-shadow: 0px 0px 5px 0px rgb(32 32 33);

        &:hover {
          box-shadow: 0px 0px 10px 0px rgba(0, 136, 255);
        }
      }

      .emoji {
        transition: 0.3s;
        width: 40px;
        min-width: 40px;
      }

      .luyin {
        color: #fff;
        margin-left: 1.5%;
        font-size: 30px;
        text-align: center;
        transition: 0.3s;
        width: 50px;
        min-width: 50px;
      }

      .inputs {
        width: 95%;
        height: 50px;
        background-color: rgb(66, 70, 86);
        border-radius: 15px;
        border: 2px solid rgb(34, 135, 225);
        padding: 10px;
        box-sizing: border-box;
        transition: 0.2s;
        font-size: 20px;
        color: #fff;
        font-weight: 100;
        margin: 0 20px;

        &:focus {
          outline: none;
        }
      }

      .send {
        background-color: rgb(48 51 59);
        border: 0;
        transition: 0.3s;
        box-shadow: 0px 0px 5px 0px rgb(32 32 33);
        &:hover {
          box-shadow: 0px 0px 10px 0px rgba(0, 136, 255);
        }
      }
    }
  }
}


.line {
  position: relative;
  width: 94%;
  margin-left: 2%;
  height: 2px;
  background: linear-gradient(to right, red, yellow, green);
  animation: shrink-and-expand 2s ease-in-out infinite;

}

.line::before,
.line::after {
  content: "";
  position: absolute;
  top: 0;
  width: 50%;
  height: 100%;
  background: inherit;
}

.line::before {
  border-top-left-radius: 2px;
  border-bottom-left-radius: 2px;
  left: 0;
  transform-origin: left;
  animation: shrink-left 2s ease-in-out infinite;
}

.line::after {
  border-top-left-radius: 2px;
  border-bottom-left-radius: 2px;
  right: 0;
  transform-origin: right;
  animation: shrink-right 2s ease-in-out infinite;
}

@keyframes shrink-and-expand {

  0%,
  100% {
    transform: scaleX(1);
  }

  50% {
    transform: scaleX(0);
  }
}

@keyframes shrink-left {

  0%,
  50% {
    transform: scaleX(1);
  }

  50.1%,
  100% {
    transform: scaleX(0);
  }
}

@keyframes shrink-right {

  0%,
  50% {
    transform: scaleX(1);
  }

  50.1%,
  100% {
    transform: scaleX(0);
  }
}</style>
