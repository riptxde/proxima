import { ref, readonly } from "vue";
import type { PageType } from "@/types/navigation";

const activePage = ref<PageType>("editor");

export function useNavigation() {
  const navigate = (page: PageType) => {
    activePage.value = page;
  };

  const isActive = (page: PageType) => {
    return activePage.value === page;
  };

  return {
    activePage: readonly(activePage),
    navigate,
    isActive,
  };
}
