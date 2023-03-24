
import { reactive, toRefs } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

interface ClipboardData {
  selectedContent: string;
}

export default function useClipboard() {
  const state = reactive({
    question: '',
  });
  const getSelectedContent = async () => {
    try {
      let selected: string = await invoke('get_selected_content')
      console.log(selected)
      const clipboardData: ClipboardData = {
        selectedContent: selected,
      }
      if (clipboardData.selectedContent === '' || clipboardData.selectedContent === state.question) {
        return Promise.reject('empty Clipboard or not change selected content');
      }
      state.question = clipboardData.selectedContent;
    } catch (error) {
      console.error(error);
    }
  };

  return {
    ...toRefs(state),
    getSelectedContent,
  };
}
