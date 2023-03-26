import { defineStore } from 'pinia'
import { CopyDocument, Check, Sunrise } from '@element-plus/icons-vue'

export const usePromptModeStore = defineStore('prompt-mode', {
  state: () => ({
    items: [
      { icon: CopyDocument, title: '解释代码', description: '作为出色的程序员, 请帮我解释这段代码.' },
      { icon: Check, title: 'Delici pizza', description: ''  },
      { icon: Sunrise, title: 'Delicioussss pizza', description: ''  },
    ],
    selectedPrompt: { icon: CopyDocument, title: '解释代码', description: '作为出色的程序员, 请帮我解释这段代码.' }
  }),
  getters: {
    getSelectedPrompt: (state) => {
        if (state.selectedPrompt.description) {
            return state.selectedPrompt.description
        }else if(state.selectedPrompt.title) {
            return state.selectedPrompt.title
        }else {
            return ''
        }
    },
  },
  actions: {
    removeByTitle(title: string) {
      this.clearSelectedPromptByTitle(title)
      const index = this.items.findIndex(item => item.title === title)
      if (index !== -1) {
        this.items.splice(index, 1)
      }
    },
    // 如果没有传递description参数且title中包含冒号
    // 那么就将冒号前的内容作为新的title
    // 将冒号后的内容作为新的description
    // 如果没有这个特殊情况，就按照原有的逻辑存储
    add(title: string, description?: string) {
        if (title.includes(':') || title.includes('：')) {
          const [newTitle, newDescription] = title.split(/[:：]/)
          this.items.push({
            icon: CopyDocument,
            title: newTitle.trim(),
            description: newDescription.trim() || description || ''
          })
        } else {
          this.items.push({
            icon: CopyDocument,
            title,
            description: description || ''
          })
        }
    },
    /// 从items中匹配title相同的item, 设置为selectedPrompt
    setSelectedPrompt(title: string) {
        const index = this.items.findIndex(item => item.title === title)
        if (index !== -1) {
          this.selectedPrompt = this.items[index]
        }
    },
    // 清除selectedPrompt
    clearSelectedPrompt() {
        this.selectedPrompt = {}
    },
    clearSelectedPromptByTitle(title: string) {
        const index = this.items.findIndex(item => item.title === title)
        if (index !== -1) {
            this.clearSelectedPrompt()
        }
    }
  }
})
