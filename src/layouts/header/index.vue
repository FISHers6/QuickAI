<template lang='pug'>
el-header#Header
  HeaderMenu
  Tools
</template>
<script lang='ts' setup>
import HeaderMenu from '@/components/Menu.vue'
import Tools from '@/components/Tools.vue'
</script>
<style lang='scss' scoped>
#Header {
  background-color: var(--color-white-003);
  border-radius: var(--radius-3);
  margin-bottom: 2rem;
  height: 3.5rem;
  display: flex;
  align-items: center;
  padding: 0 2rem;
}
</style>