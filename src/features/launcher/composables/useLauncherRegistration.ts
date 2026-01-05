import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const isCurrentLauncher = ref(false);
const isChecking = ref(false);

export function useLauncherRegistration() {
  const checkRegistration = async () => {
    isChecking.value = true;

    try {
      const status = await invoke<boolean>("launcher_check_registration");
      isCurrentLauncher.value = status;
    } catch (error) {
      console.error("Failed to check registration:", error);
      isCurrentLauncher.value = false;
    } finally {
      isChecking.value = false;
    }
  };

  return {
    // State
    isCurrentLauncher,
    isChecking,

    // Methods
    checkRegistration,
  };
}
