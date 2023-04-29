import { defineStore } from 'pinia'
import type { SettingsState } from './helper'
import { defaultSetting, getLocalState, removeLocalState, setLocalState } from './helper'

export const useSettingStore = defineStore('setting-store', {
  state: (): SettingsState => getLocalState(),
  actions: {
    updateSetting(settings: Partial<SettingsState>) {
      // 对象解构
      // 将传入的部分属性对象与响应式对象 $state 进行合并
      // 由于合并了多个对象, 如果传入的参数对象中包含多个字段, 
      // 则会同时修改这些参数对象中对应的字段
      this.$state = { ...this.$state, ...settings }
      console.log('this state',this.$state)
      this.recordState()
    },

    resetSetting() {
      this.$state = defaultSetting()
      removeLocalState()
    },

    recordState() {
      setLocalState(this.$state)
    },
  },
})
