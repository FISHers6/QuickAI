<template>
    <div class="search_actualSearchBox__DgEOi">
        <!-- <form action="" class="search_form__Mu_pF">
            <input type="text" id="search-box-input" class="search_searchInput__5Y9_W" autocomplete="off" autocapitalize="off" autocorrect="off" spellcheck="false" placeholder="Search..." value="">
        </form> -->
        <el-input v-model="question" :suffix-icon="loading ? Loading : Promotion"  placeholder="Say something & Enter ... " @keydown.stop="invokeEnter" class="search_searchInput__5Y9_W">
            <!-- <template #prepend>
                <el-select v-model="select" placeholder="Select" style="width: 70px">
                    <el-option label="Restaurant" value="1" />
                    <el-option label="Order No." value="2" />
                    <el-option label="Tel" value="3" />
                </el-select>
            </template> -->
        </el-input>
        <div class="search_searchResults__tDMVe">
            <div>
                <h3 class="search_searchSectionHeader__RGOMg">推荐使用👇</h3>
                <ul class="search_search">
                    <a href="#" class="search_searchItem__0T3l3" data-hoverable-id="browse-extensions"><svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M5.75 3.75h-1a1 1 0 0 0-1 1v1m2-2v-2m0 2h4.5m0 0h1a1 1 0 0 1 1 1v1m-2-2v-2m-4.5 12.5v-2m4.5 2v-2m2-6.5h2m-2 0v4.5m0 0v1a1 1 0 0 1-1 1h-6.5a1 1 0 0 1-1-1v-1m8.5 0h2m-10.5 0h-2m2 0v-4.5m0 0h-2" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path></svg><span>解放双手~</span></a>
                    <a href="#" class="search_searchItem__0T3l3 search_focussed__Do_iu" data-hoverable-id="create-extensions"><svg viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg"><path d="M8 3.75v8.5M12.25 8h-8.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"></path></svg><span>会话模式</span><div class="HotKey_container__WmPBz" style="right: 8px;"><i class="HotKey_key__TwJJb">↵</i></div></a>
                </ul>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import Loading from '@/components/loading.vue'
import { invoke } from '@tauri-apps/api';
import { Promotion } from '@element-plus/icons-vue'

import { ref } from 'vue'
    import { Search } from '@element-plus/icons-vue'
const question = ref('')
const select = ref('')
const loading = ref(false)

window.addEventListener('DOMContentLoaded', () => {
    const body = document.body;
    const width = body.clientWidth;
    const height = body.clientHeight;
    invoke('set_size', { payload: { width, height } });
});

const invokeEnter = () => {

}

</script>

<style scoped>

body {
    width: 420px;
}

.search_actualSearchBox__DgEOi {
    display: flex;
    flex-direction: column;
    border: 1px solid rgba(49,49,49,.6);
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

.search_searchInput__5Y9_W {
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

.search_searchResults__tDMVe {
    padding: 0 8px 16px;
    color: hsla(0,0%,100%,.4);
    font-size: 15px;
}

.search_searchSectionHeader__RGOMg {
    padding: 24px 16px 16px;
    /* padding: 0px 0px 16px; */
    font-size: 13px;
    font-weight: 500;
    -webkit-user-select: none;
    -moz-user-select: none;
    user-select: none;
}

menu, ol, ul {
    padding: 0;
    margin: 0;
    list-style: none;
}

.search_searchItem__0T3l3 {
    position: relative;
    display: flex;
    align-items: center;
    padding: 8px 16px;
    border-radius: 6px;
    background: rgba(255,255,255,var(--bg-opacity));
    cursor: pointer;
    gap: 8px;
    --bg-opacity: 0;
}

.search_searchItem__0T3l3.search_focussed__Do_iu {
    --bg-opacity: 0.1;
}

.search_searchItem__0T3l3>svg {
    height: 16px;
}

*, :after, :before {
    box-sizing: border-box;
}

a {
    color: inherit;
    text-decoration: none;
}

.search_searchItem__0T3l3>span {
    flex-grow: 1;
}

.HotKey_container__WmPBz, .HotKey_key__TwJJb {
    display: flex;
    align-items: center;
    pointer-events: none;
}

.HotKey_key__TwJJb {
    min-width: 20px;
    height: 20px;
    justify-content: center;
    padding: 0 5px;
    border-radius: 4px;
    background: hsla(0,0%,100%,.1);
    color: hsla(0,0%,100%,.6);
    font-size: 11px;
    font-style: normal;
    -webkit-user-select: none;
    -moz-user-select: none;
    user-select: none;
}

.HotKey_container__WmPBz, .HotKey_key__TwJJb {
    display: flex;
    align-items: center;
    pointer-events: none;
}

</style>