<template lang='pug'>
#header-menu
  el-tabs(v-model="activeName" class="demo-tabs" @tab-click="handleClick")
    el-tab-pane(v-for="({label, name, path}, index) in menuData", :key="name", :label="label", :name="name")
</template>
<script lang='ts' setup>
import { useRoute, useRouter } from 'vue-router';

const router = useRouter()
const route = useRoute()

const activeName = ref('chatgpt4')

const menuData = ref([{
  label: 'ChatGPT 4.0',
  name: 'chatgpt4',
}, {
  label: 'Test1',
  name: 'test1',
}, {
  label: 'Test2',
  name: 'test2',
}
])

const handleClick = (val: any) => {
  router.push({ path: `${val.props.name}` })
}

onBeforeMount(() => {
  const a: any = route.path.split('/')
  activeName.value = a[a.length - 1]
})


</script>
<style lang='scss' scoped>
#header-menu {
  width: 100%;
  height: 100%;

  :deep(.el-tabs) {

    height: 3.5rem;
    display: flex;
    align-items: center;

    .el-tabs__item {
      transition: all .3s ease;

      color: var(--color-white-040);

      &:hover {
        color: var(--color-white-080);
      }
    }

    .is-active {
      color: var(--color-purple) !important;
      font-size: 1.25rem;
      font-weight: 800;
    }

    .el-tabs__header {
      margin: 0;
    }
  }




  :deep(.el-tabs__nav-wrap::after) {
    // 
    display: none;

  }

  :deep(.el-tabs__active-bar) {
    display: none;
  }
}
</style>