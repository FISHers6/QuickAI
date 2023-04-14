<template>
<div class="iconLogo">
  <div class="iconLogo-icon">
    <template v-if="icon.type === 'image'">
      <img :src="icon.value" :style="icon.style">
    </template>
    <template v-else-if="icon.type === 'emoji'">
      <span :style="{ ...icon.style, fontSize: 'inherit' }">{{ icon.value }}</span>
    </template>
    <template v-else-if="icon.type === 'icon'">
      <i :style="icon.style"></i>
    </template>
  </div>
</div>

</template>
  
<script lang="ts">
import { defineComponent, PropType } from "vue";

interface Icon {
  type: 'image' | 'emoji' | 'icon';
  value: string;
  style?: Record<string, any>;
}

export default defineComponent({
  name: 'IconLogo',
  props: {
    icon: {
      type: Object as () => Icon,
      required: true,
    },
  },
});
</script>
  
  <style scoped>

.iconLogo {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 5px;
  width: 16px;
  height: 16px;
}

.iconLogo-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
}

.iconLogo-icon img,
.iconLogo-icon i {
  width: 100%;
  height: 100%;
  object-fit: contain;
}


</style>
  