import { defineStore } from 'pinia'
import { getLocalState, setLocalState } from './helper'
import type { Record, RecordState} from './helper'

export const useRecordStore = defineStore('record-store', {
  state: (): RecordState => getLocalState(),

  getters: {
    getRecord(state: RecordState) {
      return (uuid?: number) => {
        if (uuid)
          return state.records.find(item => item.uuid === uuid)?.data ?? []
        return []
      }
    },

    getLatestRecordID(state: RecordState) {
      return () => {
        const maxUuid = state.records.reduce((max, current) => Math.max(max, current.uuid || 0), 0);
        return maxUuid
      }
    },
  },

  actions: {
    getRecordMessages(recordId: number) {
      if (!recordId || recordId === 0) {
        console.log('recordId is 0 or nuot found')
        return null
      }
      const chatIndex = this.records.findIndex(item => item.uuid === recordId)
      console.log(chatIndex)
      if (chatIndex !== -1)
        return this.records[chatIndex].data.filter(item => item.text !== '')
      return null
    },

    addRecordMessage(uuid: number, record: Record) {
      if (!uuid || uuid === 0) {
        return
      }

      const index = this.records.findIndex(item => item.uuid === uuid)
      if (index == -1) {
        this.records.push({ uuid: uuid, data: [record]})
      }else {
        this.records[index].data.push(record)
        this.recordState()
      }
    },

    deleteRecord(uuid: number) {
      if (!uuid || uuid === 0) {
        return
      }

      const recordIndex = this.records.findIndex(item => item.uuid === uuid)
      if (recordIndex !== -1) {
        this.records[recordIndex].data.splice(recordIndex, 1)
        this.recordState()
      }
    },

    recordState() {
      setLocalState(this.$state)
    },
  },
})
