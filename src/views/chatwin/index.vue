<template lang='pug'>
ChatWindow(ref="chatWindow" :frinedInfo="chatWindowInfo" :settingInfo="SettingInfo" :storeStatu="storeStatus")
</template>
    
<script lang='ts' setup>
import ChatWindow from '@/components/chatWindow.vue'

const chatWindowInfo = ref({
      img: "",
      name: "ChatGPT-3.5",
      detail: "chatgpt v3.5 所基于的模型",
      lastMsg: "chatgpt v3.5 所基于的模型",
      id: "gpt-3.5-turbo",
    //   headImg: AI_HEAD_IMG_URL,
      headImg: '../x.jpg',
      showHeadImg: true
})

const storeStatus = ref(0)

const SettingInfo = ref({
        translateEnglish: false,
        openProductionPicture: false,
        openChangePicture: false,
        KeyMsg: '213213',
        // KeyMsg: process.env.VUE_APP_OPENAI_API_KEY,
        MaxTokens: 1000,
        Temperature: 1,
        TemperatureAudio: 0,
        TopP: 1,
        FrequencyPenalty: 0,
        PresencePenalty: 0,
        n: 1,
        size: "256x256",
        language: "zh",
        contentImageUrl: "",
        fineTunes: {
          training_file: "",
          validation_file: undefined,
          model: "curie",
          n_epochs: 4,
          batch_size: undefined,
          learning_rate_multiplier: undefined,
          prompt_loss_weight: 0.01,
          compute_classification_metrics: false,
          classification_n_classes: undefined,
          classification_positive_class: undefined,
          classification_betas: undefined,
          suffix: ""
        }
})

// const personList = ref<string []>([])

// const personCardSort = (id: string) => {
//       if (id !== personList[0].id) {
//         console.log(id);
//         let nowPersonInfo;
//         for (let i = 0; i < personList.length; i++) {
//           if (personList[i].id == id) {
//             nowPersonInfo = personList[i];
//             personList.splice(i, 1);
//             break;
//           }
//         }
//         personList.unshift(nowPersonInfo);
//       }
// }

</script>
    
<style lang='scss' scoped>
h1 {
    color: var(--color-white-100);
}
</style>