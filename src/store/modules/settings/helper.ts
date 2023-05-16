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
  mode: string
  quickAskShortcut: string,
  searchShortcut: string,
  chatShortcut: string,
  enableSelect: boolean,
}

export interface AppConfig {
  quickAskShortcut: String | null,
  searchShortcut: String | null,
  chatShortcut: String | null,
  mode: String | null,
  isDarkMode: boolean,
  language: String,
  apiKey: String,
  proxy: String,
  useChatContext: boolean,
  enableSelect: boolean,
}

export function defaultSetting(): SettingsState {
  return {
    systemMessage: '',
    language: 'zh',
    apiKey: '',
    proxy: 'https://proxy3.pages.dev',
    isDarkMode: true,
    useChatContext: true,
    conversationRequest: {
      conversationId: '',
      parentMessageId: ''
    },
    mode: '快捷提问',
    quickAskShortcut: "Shift+D",
    searchShortcut: "CommandOrControl+Shift+Space",
    chatShortcut: "Shift+C",
    enableSelect: true,
  }
}

export function getLocalState(): SettingsState {
  const localSetting: SettingsState | undefined = ss.get(LOCAL_NAME)
  return { ...defaultSetting(), ...localSetting }
}

export function setLocalState(setting: SettingsState): void {
  console.log('setLocal',setting)
  ss.set(LOCAL_NAME, setting)
}

export function removeLocalState() {
  ss.remove(LOCAL_NAME)
}
