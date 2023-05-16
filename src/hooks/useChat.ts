import { useChatStore } from '@/store'
import { Chat } from '@/typings/chat'
export function useChat() {
  const chatStore = useChatStore()

  const getChatByUuidAndIndex = (uuid: number, index: number) => {
    return chatStore.getChatByUuidAndIndex(uuid, index)
  }

  const getChatMessages = (chatID: number) => {
    console.log('getChatMessages chatId', chatID)
    return chatStore.getChatMessages(chatID)
  }

  const addChat = (uuid: number, chat: Chat.Chat) => {
    chatStore.addChatByUuid(uuid, chat)
  }

  const updateChat = (uuid: number, index: number, chat: Chat.Chat) => {
    chatStore.updateChatByUuid(uuid, index, chat)
  }

  const updateChatSome = (uuid: number, index: number, chat: Partial<Chat.Chat>) => {
    chatStore.updateChatSomeByUuid(uuid, index, chat)
  }

  const clearChatMessage = (chatID: number) => {
    console.log('clearChatMessage chatId', chatID)
    chatStore.clearChatMessage(chatID)
  }

  return {
    addChat,
    getChatMessages,
    updateChat,
    updateChatSome,
    getChatByUuidAndIndex,
    clearChatMessage,
  }
}
