<script setup lang="ts">
import { Promotion, WarnTriangleFilled, EditPen } from '@element-plus/icons-vue'
import { invoke } from "@tauri-apps/api/tauri";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template lang="pug">
.says-area
  .empty-area(v-if="!greetMsg")
    el-icon.icon
      EditPen
  span {{ greetMsg }}
//- .greet
  .card
    input( id="greet-input" v-model="name" placeholder="Enter a name...")
    button(type="button" @click="greet") Greet
  p {{ greetMsg }} 
</template>

<style lang="scss" scoped>
.says-area {
  height: 30vh;
  background-color: var(--el-fill-color-blank);
  border-radius: var(--radius-1);
  padding: 5px 11px;
  box-sizing: border-box;
  color: var(--color-white-080);

  .empty-area {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .icon {
      font-size: 3rem;
      color: var(--color-white-003);
    }
  }
}
</style>
