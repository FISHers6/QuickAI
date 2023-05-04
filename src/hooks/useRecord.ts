import { useRecordStore } from '@/store'
import type { Record } from '@/store/modules/record/helper'
export function useRecord() {
  const recordStore = useRecordStore()

  const getRecordMessages = (uuid: number) => {
    return recordStore.getRecordMessages(uuid)
  }

  const addRecordMessage = (uuid: number, record: Record) => {
    recordStore.addRecordMessage(uuid, record)
  }

  const deleteRecord = (uuid: number) => {
    recordStore.deleteRecord(uuid)
  }

  return {
    getRecordMessages,
    addRecordMessage,
    deleteRecord
  }
}
