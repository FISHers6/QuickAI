<template>
  <div>
    <h3 class="searchSectionHeader">{{ title }}</h3>
    <ul class="searchItemList">
      <li
        v-for="(item, index) in items"
        :key="item.id"
        :class="{ searchItem: true, search_focussed: index === selectedIndex }"
        @mouseover="selectItem(index)"
        @click="handleClickItem(item)"
      >
        <IconLogo class="Item_Logo" :icon="item.icon" />
        <span>{{ item.name }}</span>
        <IconLogo
          class="HotKey_key_bg" 
          v-show="index === selectedIndex" 
          v-for="(icon, idx) in item.tagIcon" :key="idx" :icon="icon" 
        />
      </li>
    </ul>
  </div>
</template>
  
<script lang="ts">
import { defineComponent, ref } from "vue";
import IconLogo from "@/components/IconLogo.vue";
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

export default defineComponent({
  name: "SearchList",
  components: {
    IconLogo,
  },
  props: {
    title: {
      type: String,
      required: true,
    },
    items: {
      type: Array as () => SearchItem[],
      required: true,
    },
  },
  emits: ['click-item'], // 声明 emit 事件
  setup(props, context) {
    const selectedIndex = ref(-1);

    function selectItem(index: number) {
      selectedIndex.value = index;
    }

    const handleClickItem = (item: SearchItem) => {
      context.emit('click-item', item)
    }

    return {
      selectedIndex,
      selectItem,
      handleClickItem
    };
  },
});
</script>
  
<style scoped>
.searchSectionHeader {
  padding: 16px 16px 16px;
  /* padding: 0px 0px 16px; */
  font-size: 13px;
  font-weight: 500;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}

.searchItem {
  position: relative;
  display: flex;
  align-items: center;
  padding: 8px 16px;
  border-radius: 6px;
  background: rgba(255, 255, 255, var(--bg-opacity));
  cursor: pointer;
  gap: 8px;
  --bg-opacity: 0;
}

.searchItem.search_focussed {
  --bg-opacity: 0.1;
}

.searchItem > img {
  height: 16px;
}

*,
:after,
:before {
  box-sizing: border-box;
}

a {
  color: inherit;
  text-decoration: none;
}

.searchItem > span {
  flex-grow: 1;
}

menu,
ol,
ul {
  padding: 0;
  margin: 0;
  list-style: none;
}

.Item_Logo {
  min-width: 25px;
  height: 25px;
  justify-content: center;
  padding: 0 5px;
  border-radius: 4px;
  background: hsla(0, 0%, 100%, 0.1);
  color: hsla(0, 0%, 100%, 0.6);
  font-style: normal;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}

.HotKey_key_bg {
  min-width: 20px;
  height: 20px;
  justify-content: center;
  padding: 0 5px;
  border-radius: 4px;
  background: hsla(0, 0%, 100%, 0.1);
  color: hsla(0, 0%, 100%, 0.6);
  font-style: normal;
  font-size: 10px;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}

.HotKey_key_bg:not(:last-child) {
  margin-right: 1px;
}

.HotKey_container,
.Item_Logo {
  display: flex;
  align-items: center;
  pointer-events: none;
}
</style>