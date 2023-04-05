import { useSettingStore } from '@/store'
import type { SettingsState } from '@/store/modules/settings/helper'
import { getLocalState } from '@/store/modules/settings/helper'
 
export function useSettings() {
  const settingStore = useSettingStore()

  const updateSetting = (settings: Partial<SettingsState>) => {
    settingStore.updateSetting(settings)
  }

  const resetSetting = () => {
    settingStore.resetSetting()
  }

  const getSetting = ():SettingsState => {
    return getLocalState()
  }

  return {
    getSetting,
    updateSetting,
    resetSetting,
  }
}
