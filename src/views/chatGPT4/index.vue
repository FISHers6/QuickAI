<template lang='pug'>
#ChatGPT4
  .yq
    h4 Your question is:
    el-input.textarea(:suffix-icon="loading ? Loading : Promotion" v-model="question" placeholder="Say something & Enter ... " @keydown.stop="askTheQuestion")
  .yq
    h4 Your Prompt is:
    el-autocomplete(v-model="state" :fetch-suggestions="querySearch" popper-class="my-autocomplete" placeholder="Ask ChatGPT. Ex: Write an email reply in yoda style" @select="handleSelect")
      template(#suffix)
        el-icon.el-input__icon(@click="handleIconClick")
          edit
      template(#default="{ item }")
        div.value {{ item.value }}
        span.link {{ item.link }}
    .prompts
      el-tag(v-for="tag in promptsTags", effect="plain", :key="tag", closable, :disable-transitions="false", @close="handleClose(tag)") {{ tag }}
      el-input.add-tag(v-if="inputVisible", ref="InputRef", v-model="inputValue", size="small" @keyup.enter="handleInputConfirm", @blur="handleInputConfirm")
      el-button(v-else class="button-new-tag ml-1" size="small" @click="showInput") + New Tag
  .chats-says
    h4 ChatGPT says:
    Result(:response="answer" :loading="loading")
    
</template>

<script lang='ts' setup>
import Result from '@/components/Result.vue'
import Loading from '@/components/loading.vue'
import type { ElInput } from 'element-plus'
import { Edit, Promotion } from '@element-plus/icons-vue'
import askChatGPT from '@/hooks/api'
import GPTParam from '@/hooks/api'

const question = ref('')
const answer = ref('')
const loading = ref(false)

const inputValue = ref('')
const promptsTags: any = ref([ 1, 2, 3 ])
const inputVisible = ref(false)
const InputRef = ref<InstanceType<typeof ElInput>>()

const handleClose = (tag: string) => promptsTags.value.splice(promptsTags.value.indexOf(tag), 1)

const showInput = () => {
  inputVisible.value = true
  nextTick(() => InputRef.value!.input!.focus())
}

const handleInputConfirm = () => {
  if (inputValue.value) promptsTags.value.push(inputValue.value)
  inputVisible.value = false
  inputValue.value = ''
}


const askTheQuestion = async ({ isComposing, key }: KeyboardEvent) => {
  // 解决中文输入法回车时触发的bug
  if (!isComposing && key === 'Enter' && !loading.value) {
    loading.value = true
    console.log('ask starting...')
    answer.value = ''
    let AskGPTParam = {
      question: question.value,
      prompts: '',
      apiKey: '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'
    }
    await askChatGPT(AskGPTParam, answer, loading)
    console.log('ask start end.')
  }
}


interface LinkItem {
  value: string
  link: string
}

const state = ref('')
const links = ref<LinkItem[]>([])

const querySearch = (queryString: string, cb: any) => {
  const results = queryString
    ? links.value.filter(createFilter(queryString))
    : links.value
  // call callback function to return suggestion objects
  cb(results)
}
const createFilter = (queryString: any) => {
  return (restaurant: any) => {
    return (
      restaurant.value.toLowerCase().indexOf(queryString.toLowerCase()) === 0
    )
  }
}
const loadAll = () => {
  return [
    { value: 'vue', link: 'https://github.com/vuejs/vue' },
    { value: 'element', link: 'https://github.com/ElemeFE/element' },
    { value: 'cooking', link: 'https://github.com/ElemeFE/cooking' },
    { value: 'mint-ui', link: 'https://github.com/ElemeFE/mint-ui' },
    { value: 'vuex', link: 'https://github.com/vuejs/vuex' },
    { value: 'vue-router', link: 'https://github.com/vuejs/vue-router' },
    { value: 'babel', link: 'https://github.com/babel/babel' },
  ]
}
const handleSelect = (item: LinkItem) => {
  console.log(item)
}

const handleIconClick = (ev: Event) => {
  console.log(ev)
}

onMounted(() => {
  links.value = loadAll()
})

</script>

<style lang='scss' scoped>
#ChatGPT4 {
  color: var(--color-white-100);

  &>* {
    margin-bottom: 1.628rem;
    &:last-child {
      margin-bottom: 0;
    }
  }

  .yq,
  .chats-says {
    h4 {
      margin-bottom: 1rem;
    }
  }

  .prompts {
    display: flex;
    flex-wrap: wrap;
    margin-top: 0.5rem;

    .el-tag {
      background-color: #915eff34;
    }

    .el-input.add-tag {
      width: 6rem !important;
      height: 24px;
    }

    &>* {
      margin-right: 0.5rem;
      margin-bottom: 0.5rem;

      &:last-child {
        margin-right: 0;
      }
    }

    .el-button {
      border-radius: var(--el-border-radius-base) !important;
      border-color: var(--el-button-text-color);
    }

    .el-button:focus,
    .el-button:hover {
      color: var(--el-color-primary);
      border-color: var(--el-color-primary);
      background-color: #915eff34;
      outline: 0;
    }
  }
}
</style>