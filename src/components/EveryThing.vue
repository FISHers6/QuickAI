<template>
    <div class="SearchBox">
        <!-- <el-icon><Search style="color: white;"/></el-icon> -->
        <!-- 添加图标click事件 -->
        <el-input  class="searchInput" v-model="question" :suffix-icon="loading ? Loading : Promotion"  placeholder="Say something & Enter ... " @keydown.enter.stop="invokeEnter"></el-input>
        <div class="searchResults">
            <SearchList v-if="showResults" :title="searchText" :items="searchResults" @click-item="clickItem" />
        </div>
    </div>
</template>

<script lang="ts" setup>
import Loading from '@/components/loading.vue'
import { invoke } from '@tauri-apps/api';
import { Promotion } from '@element-plus/icons-vue'
import { ref } from 'vue'
import { Search } from '@element-plus/icons-vue'
import SearchList from '@/components/SearchList.vue'
import smilingFaceWithSunglasses from "@/assets/img/emoji/smiling-face-with-sunglasses.png"


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

const question = ref('')
const select = ref('')
const loading = ref(false)

const searchText = ref('推荐使用')
const showResults = computed(() => searchText.value.length > 0);

const searchResults = ref([
    {
        id: 1,
        name: '解放双手 (默认)',
        icon: {
            type: 'emoji',
            value: '👌',
        },
        tagIcon:  [
            {
                type: 'emoji',
                value: '↵',
            },
        ]
    },
    {
        id: 2,
        name: '对话模式',
        icon: {
            type: 'emoji',
            value: '😋',
        },
        tagIcon:  [
        {
            type: 'emoji',
            value: '👉',
        },
        ]
    },
    {
        id: 3,
        name: '快捷问答',
        icon: {
            type: 'emoji',
            value: '🚀',
        },
        tagIcon:  [
        {
            type: 'emoji',
            value: '👉',
        },
        ]
    },
    {
        id: 4,
        name: '询问桌面看板娘',
        icon: {
            type: 'image',
            value: smilingFaceWithSunglasses,
        },
        tagIcon:  [{
            type: 'emoji',
            value: '😍',
        },]
    }
])

const clickItem = (item: SearchItem) => {
    console.log('click', item)
}

window.addEventListener('DOMContentLoaded', () => {
    const body = document.body;
    const width = body.clientWidth;
    const height = body.clientHeight;
    invoke('set_size', { payload: { width, height } });
});

const invokeEnter = () => {
    console.log('enter', question)
}

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
    /* border: 1px solid rgba(49,49,49,.6); */
    /* border-radius: 15px; 由于窗口磨砂效果, 颜色未生效 */
    /* margin: 128px auto; */
    /* background: #161616; */
    /* width: 400px; */
    width: 100%;
}

:deep(.el-input__inner){
 color: rgb(196, 193, 193) !important;
}

.search_form__Mu_pF {
    position: relative;
    display: flex;
}

.searchInput {
    flex-grow: 1;
    padding: 16px;
    border: none;
    border-bottom: 1px solid hsla(0,0%,100%,.1);
    background: transparent;
}
button, input, optgroup, select, textarea {
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
    color: hsla(0,0%,100%,.4);
    font-size: 15px;
}


</style>