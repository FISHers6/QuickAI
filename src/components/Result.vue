<template>
    <div class="result-container">
        <el-row justify="space-between" class="say-tip">
          <div>ChatGPT Says:</div>
          <div class="flex-grow" />
          <div>
            <el-icon><Refresh /></el-icon>
            <el-icon class="copy-icon"><CopyDocument /></el-icon>
          </div>
        </el-row>
        <el-row justify="start">
          <el-skeleton v-if="loading" :rows="4" animated/>
          <el-text v-else class="mx-1 result-text">{{response}}</el-text>
        </el-row>
      </div>
</template>

<script lang="ts">
import { PropType, defineComponent } from "vue";

import { CopyDocument, Refresh } from '@element-plus/icons-vue'
import { ElIcon, ElRow } from "element-plus";

export default defineComponent({
    name: 'Result',
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
        ElIcon,
        ElRow,
        Refresh,
        CopyDocument
    }
})

</script>


<style>
.flex-grow {
    flex-grow: 1;
    display: inline-block;
}


.copy-icon {
    margin-left: 6px;
}

.result-container {
    border-top-width: 1px;
    --tw-border-opacity: 1;
    border-top-color: rgb(203 213 225/var(--tw-border-opacity));
    margin-top: 10px;
    padding-top: 12px;
    padding-bottom: 8px;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: stretch;
}
  
.say-tip {
    font-weight: 500;
    font-size: 18px;
    line-height: 28px;
    --tw-text-opacity: 1;
    color: rgb(100 116 139/var(--tw-text-opacity));
    margin-bottom: 5px;
}

.result-text {
    padding-left:2px;
    font-size: 15px;
    line-height: 24px;
}

</style>