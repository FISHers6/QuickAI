<template lang='pug'>
el-container#Layouts.titlebar(data-tauri-drag-region)
  Header
  Main
</template>
<script lang='ts' setup>
import Header from './header/index.vue'
import Main from './main/index.vue'
</script>
<style lang='scss' scoped>
#Layouts{
  width: 100%;
  height: 100vh;
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
}
</style>