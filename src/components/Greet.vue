<template lang="pug">
.says-area
  .empty-area(v-if="!response")
    el-icon.icon
      EditPen
  span {{ response }}
//- .greet
  .card
    input( id="greet-input" v-model="name" placeholder="Enter a name...")
    button(type="button" @click="greet") Greet
  p {{ response }} 
</template>

<script lang="ts">
import { Promotion, WarnTriangleFilled, EditPen } from '@element-plus/icons-vue'

export default defineComponent({
    name: 'Greet',
    props: {
        response: {
            type: String as PropType<string>,
            required: true
        },
        loading: {
            type: Boolean as PropType<boolean>,
            required: true
        }
    },
    components: {
      Promotion, WarnTriangleFilled, EditPen
    }
})

</script>

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
