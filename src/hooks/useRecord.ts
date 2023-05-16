import { useRecordStore } from '@/store'
import type { Record } from '@/store/modules/record/helper'
export function useRecord() {
  const recordStore = useRecordStore()

  const getRecordMessages = (uuid: string) => {
    return recordStore.getRecordMessages(uuid)
  }

  const addRecordMessage = (uuid: string, record: Record) => {
    recordStore.addRecordMessage(uuid, record)
  }

  const deleteRecord = (uuid: string) => {
    recordStore.deleteRecord(uuid)
  }

  return {
    getRecordMessages,
    addRecordMessage,
    deleteRecord
  }
}
