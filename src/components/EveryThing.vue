<template>
  <div class="SearchBox" data-tauri-drag-region>
    <!-- <el-input class="searchInput" v-model="question" :suffix-icon="loading ? Loading : Promotion"  placeholder="Say something & Enter ... " @keydown.enter.stop="invokeEnter"></el-input> -->
    <multiInput class="searchInput" @inputValue="onInputValue" :loading="loading"  @enterPressed="enterPressedHandle"></multiInput>
    <!-- <div class="searchResults">
            <SearchList v-if="showResults" :title="searchText" :items="searchResults" @click-item="clickItem" />
        </div> -->
  </div>
</template>

<script lang="ts" setup>
import Loading from "@/components/loading.vue";
import { invoke } from "@tauri-apps/api";
import { Promotion } from "@element-plus/icons-vue";
import { ref, Ref } from "vue";
import { Search } from "@element-plus/icons-vue";
import SearchList from "@/components/SearchList.vue";
import smilingFaceWithSunglasses from "@/assets/img/emoji/smiling-face-with-sunglasses.png";
import { askChatGPTV2 } from "@/hooks/useAPI";
import type { GPTParamV2 } from "@/hooks/useAPI";
import type { GPTResponse } from "@/hooks/useAPI";
import multiInput from "@/components/multiInput/index.vue";
import { onMounted, watchEffect  } from "vue";
import { ElMessage } from "element-plus";
import MultiInputValue from "@/typings/input"
import { useSettings } from '@/hooks/useSettings'
import type { SettingsState } from '@/store/modules/settings/helper'

const { updateSetting, getSetting } = useSettings()

interface Icon {
  type: "image" | "emoji" | "icon";
  value: string;
  style?: Record<string, any>;
}

interface SearchItem {
  id: number;
  name: string;
  icon: Icon;
  tagIcon: Icon[];
}

const question = ref("");
const select = ref("");
const loading = ref(false);
const answer = ref("");
// 消息发送控制停止
let controller = new AbortController();

const searchText = ref("推荐使用");
const showResults = computed(() => searchText.value.length > 0);

// not used
const searchResults = ref([
  {
    id: 1,
    name: "解放双手 (默认)",
    icon: {
      type: "emoji",
      value: "👌",
    },
    tagIcon: [
      {
        type: "emoji",
        value: "↵",
      },
    ],
  },
  {
    id: 2,
    name: "对话模式",
    icon: {
      type: "emoji",
      value: "😋",
    },
    tagIcon: [
      {
        type: "emoji",
        value: "👉",
      },
    ],
  },
  {
    id: 3,
    name: "快捷提问",
    icon: {
      type: "emoji",
      value: "🚀",
    },
    tagIcon: [
      {
        type: "emoji",
        value: "👉",
      },
    ],
  },
  {
    id: 4,
    name: "询问桌面看板娘",
    icon: {
      type: "image",
      value: smilingFaceWithSunglasses,
    },
    tagIcon: [
      {
        type: "emoji",
        value: "😍",
      },
    ],
  },
]);

// not used
const clickItem = async (item: SearchItem) => {
  console.log("click", item);
  if (item.name.includes("解放双手") || item.name.includes("自动输入")) {
    await invokeEnter(null);
  } else if (item.name === "对话模式") {
    if (question.value.trim().length > 0) {
      invoke("run_chat_mode", { payload: { question: question.value } });
    }
  } else if (item.name === "快捷提问") {
    if (question.value.trim().length > 0) {
      invoke("run_quick_answer", { payload: { question: question.value } });
    } else {
      // show tip
    }
  }
};

window.addEventListener("DOMContentLoaded", () => {
  const body = document.body;
  const width = body.clientWidth;
  const height = body.clientHeight;
  console.log(width, height);
  invoke("set_size", { payload: { width, height } });
});

window.addEventListener("resize", () => {
  const { innerWidth: width, innerHeight: height } = window;
  console.log("innerWidth", innerWidth);
  console.log("innerHeight", innerHeight);
  console.log("clientHeight", document.body.clientHeight);
  console.log("clientWidth", document.body.clientWidth);
  invoke("set_size", { payload: { width, height } });
});

// 获取 body 元素
const body = document.getElementsByTagName("body")[0];

// not used
// 创建 MutationObserver 对象
const observer = new MutationObserver((mutations) => {
  // 监听 body 元素的尺寸变化
  const bodyWidth = body.clientWidth;
  const bodyHeight = body.clientHeight;
  console.log(`Body size changed to: ${bodyWidth}x${bodyHeight}`);
});

// 配置 MutationObserver，指定观察 body 元素的子节点变化
const config = { childList: true, subtree: true };

onMounted(() => {
  observer.observe(body, config);
});

const onInputValue = (multiInputValue: MultiInputValue) => {
  console.log('onInputValue', multiInputValue)
  question.value = multiInputValue.input + multiInputValue.fileList.map((file) => `${file.fileName}-${file.fileContent}`)
}

const enterPressedHandle = async (multiInputValue: MultiInputValue | null) => {
  if(!question) {
    return
  }
  const userQuestionWithFile = question.value.trim()
  if(userQuestionWithFile.length <= 0) {
    return
  }
  const mode = getSetting().mode
  console.log('mode', mode)
  if(mode === '自动输入') {
    // 自动输入
    await invokeEnter(multiInputValue)
  }else if (mode === '对话模式') {
    // 对话模式
    invoke("run_chat_mode", { payload: { question: userQuestionWithFile } });
  }else {
    // 快捷提问
    invoke("run_quick_answer", { payload: { question: userQuestionWithFile } });
  }
}


const invokeEnter = async (multiInputValue: MultiInputValue | null) => {
  console.log('invokeEnter', multiInputValue)
  console.log("enter", question);
  loading.value = true;
  answer.value = "";
  // 0.查看配置, 判断enter键对应的项, 决定使用哪个功能模块, 目前不做, 默认走只做自动输入
  // a.以下是对于解放双手自动输入模块的逻辑
  //  1.请求 chatgpt api 设置回调, api设置的更通用, 调用方通过回调完成操作, 无需传递非共有参数
  //  2.请求失败时, 给出错误异常弹框, 不进行第三步
  //  3.第一段结果出现时, 设置前台展示, 搜索窗口隐藏不展示, spawn一个异步任务处理前后台操作, 异步sleep一段时间, 上屏第一段result
  //  4.每产出一段result, buffer/Tokenlizer层进行缓存分词处理(目前可以不做, 逻辑放在Rust层完成), 调度层发送给tauri展示

  const param: GPTParamV2 = {
    question: question.value,
    prompts: "",
    controller: controller,
  };
  let hasError = false;

  const callback = (response: GPTResponse) => {
    if (loading.value) {
      loading.value = false;
      console.log(response);
      if (response) {
        answer.value = response.content;
        try {
          let response = answer.value;
          invoke("run_auto_input", { payload: { response } });
        } catch (error: any) {
          hasError = true;
          loading.value = false;
          console.log(error);
          controller.abort();
        }
        console.log("first");
        console.log(answer.value);
      }
    } else {
      if (response) {
        answer.value = response.content;
        try {
          let response = answer.value;
          invoke("send_auto_input_value", { payload: { response } });
          question.value = "";
        } catch (error: any) {
          hasError = true;
          loading.value = false;
          console.log(error);
          controller.abort();
        }
        console.log(answer.value);
      }
    }
  };

  const errorCallback = (error: any) => {
    console.log(error);
    controller.abort();
    loading.value = false;
    hasError = true;
    ElMessage({
      message: error,
      type: "warning",
    });
  };

  // 文本对话
  await askChatGPTV2(param, callback, errorCallback);
  // if(!hasError && answer.value.length > 0) {
  // let response = answer.value
  // invoke('send_auto_input_value', { payload: { response } })
  // 设置粘贴板
  // ElMessage({
  //     message: '已将回答拷贝到粘贴板~',
  //     type: 'success',
  // })
  // }
};
</script>

<style scoped>
.example-showcase .el-dropdown-link {
  cursor: pointer;
  color: var(--el-color-primary);
  display: flex;
  align-items: center;
}
body {
  width: 420px;
}

.SearchBox {
  display: flex;
  flex-direction: column;
  width: 100%;
}

:deep(.el-input__inner) {
  color: rgb(196, 193, 193) !important;
}

.search_form__Mu_pF {
  position: relative;
  display: flex;
}

button,
input,
optgroup,
select,
textarea {
  padding: 0;
  margin: 0;
  color: inherit;
  font-family: inherit;
  font-size: 100%;
  font-weight: inherit;
  line-height: inherit;
}

.searchResults {
  padding: 0 8px 16px;
  color: hsla(0, 0%, 100%, 0.7);
  font-size: 15px;
}

</style>