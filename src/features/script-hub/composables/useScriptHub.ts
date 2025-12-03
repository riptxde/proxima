import { ref, computed } from "vue";
import type {
  Script,
  ScriptSearchResponse,
  ScriptSearchParams,
} from "../types/script";
import { useLogger } from "@/composables/useLogger";

const API_BASE_URL = "https://scriptblox.com/api/script";

const scripts = ref<Script[]>([]);
const isLoading = ref(false);
const error = ref<string | null>(null);
const currentPage = ref(1);
const totalPages = ref(0);
const searchParams = ref<ScriptSearchParams>({
  q: "",
  page: 1,
  max: 20,
  sortBy: "updatedAt",
  order: "desc",
  strict: true,
});

export function useScriptHub() {
  const { addLog } = useLogger();

  const fetchScripts = async () => {
    isLoading.value = true;
    error.value = null;

    try {
      const params = new URLSearchParams();

      Object.entries(searchParams.value).forEach(([key, value]) => {
        if (value !== undefined && value !== null && value !== "") {
          params.append(key, String(value));
        }
      });

      const endpoint = searchParams.value.q ? "search" : "fetch";
      const response = await fetch(
        `${API_BASE_URL}/${endpoint}?${params.toString()}`,
      );

      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }

      const data: ScriptSearchResponse = await response.json();

      if ("message" in data) {
        throw new Error((data as any).message);
      }

      scripts.value = data.result.scripts;
      totalPages.value = data.result.totalPages;
      currentPage.value = searchParams.value.page || 1;
    } catch (err) {
      error.value =
        err instanceof Error ? err.message : "Failed to fetch scripts";
      addLog("error", `Error fetching scripts: ${error.value}`);
    } finally {
      isLoading.value = false;
    }
  };

  const updateSearchParams = (params: Partial<ScriptSearchParams>) => {
    searchParams.value = {
      ...searchParams.value,
      ...params,
    };
    fetchScripts();
  };

  const setPage = (page: number) => {
    updateSearchParams({ page });
  };

  const setSearch = (query: string) => {
    updateSearchParams({ q: query, page: 1 });
  };

  const resetFilters = () => {
    searchParams.value = {
      q: "",
      page: 1,
      max: 20,
      sortBy: "updatedAt",
      order: "desc",
      strict: true,
    };
    fetchScripts();
  };

  const hasNextPage = computed(() => currentPage.value < totalPages.value);
  const hasPrevPage = computed(() => currentPage.value > 1);

  return {
    scripts,
    isLoading,
    error,
    currentPage,
    totalPages,
    searchParams,
    fetchScripts,
    updateSearchParams,
    setPage,
    setSearch,
    resetFilters,
    hasNextPage,
    hasPrevPage,
  };
}
