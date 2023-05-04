import { ss } from '@/utils/storage'
import { Chat } from '@/typings/chat.d'
const LOCAL_NAME = 'recordStorage'

export interface Record {
    dateTime: string
    text: string
    bot: boolean
    conversationOptions?: Chat.ConversationRequest
}

export interface RecordState {
    records: { uuid: number; data: Record[] }[]
}

export function defaultState(): RecordState {
    const uuid = 1002
    return {
      records: [{ uuid, data: [] }],
    }
  }
  
  export function getLocalState(): RecordState {
    const localState = ss.get(LOCAL_NAME)
    return { ...defaultState(), ...localState }
  }


export function setLocalState(state: RecordState) {
  ss.set(LOCAL_NAME, state)
}
