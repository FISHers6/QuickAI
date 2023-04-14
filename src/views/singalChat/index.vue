<template lang='pug'>
ChatMode(data-tauri-drag-region)
</template>
        
<script lang='ts' setup>
import { ChatMode } from "@/components/index";

</script>
        
<style lang='scss'>
@import url(@/assets/font/iconfont.css);

h1 {
  color: var(--color-white-100);
}

.chat-window {
  // display: flex !important;
  // align-items: center !important; /* 垂直居中 */
  // justify-content: center !important; /* 水平居中 */
  width: 100% !important;
  height: 100vh !important;
  margin: 0 !important;
}

.chat-window .botoom {
  height: 100vh !important;
}

.iconfont {
  font-family: "iconfont" !important;
  font-style: normal;
  font-size: 25px;
  vertical-align: middle;
  color: rgb(117,120,137);
  transition: .3s;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

</style>