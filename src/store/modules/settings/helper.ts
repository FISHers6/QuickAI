import { ss } from '@/utils/storage'
import { Chat } from '@/typings/chat.d'

const LOCAL_NAME = 'settingsStorage'

export interface SettingsState {
  systemMessage: string
  language: string
  apiKey: string
  proxy: string
  isDarkMode: boolean
  useChatContext: boolean
  conversationRequest?: Chat.ConversationRequest
}

export function defaultSetting(): SettingsState {
  return {
    systemMessage: '',
    language: 'zh',
    apiKey: '',
    proxy: '',
    isDarkMode: true,
    useChatContext: true,
    conversationRequest: {
      conversationId: '',
      parentMessageId: ''
    }
  }
}

export function getLocalState(): SettingsState {
  const localSetting: SettingsState | undefined = ss.get(LOCAL_NAME)
  return { ...defaultSetting(), ...localSetting }
}

export function setLocalState(setting: SettingsState): void {
  ss.set(LOCAL_NAME, setting)
}

export function removeLocalState() {
  ss.remove(LOCAL_NAME)
}
