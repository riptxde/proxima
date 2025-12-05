import { ref } from "vue";

const showLogs = ref(false);

export function useEditorLogs() {
  const toggleLogs = () => {
    showLogs.value = !showLogs.value;
  };

  return {
    showLogs,
    toggleLogs,
  };
}
