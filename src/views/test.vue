<template lang='pug'>
#test
  multiInput(@inputValue="onInputValue")
</template>
<script lang='ts' setup>
import multiInput from '@/components/multiInput/index.vue'

const onInputValue = (val: any) => {
  console.log('onInputValue', val)
}
</script>
<style lang='scss' scoped>
#test {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  // background-color: black;
}

</style>