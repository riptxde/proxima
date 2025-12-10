import explorerPlaceholder from "@/assets/explorer_placeholder.png";

const ROBLOX_ICON_BASE_URL =
  "https://assets.create.roblox.com/docs/e3a7237f884dc99e91505be6ce3357acabe873f7/assets/engine-reference-icons";

// Track failed icon loads to avoid retrying
const failedIcons = new Set<string>();

/**
 * Composable for managing Roblox instance icons
 */
export function useExplorerIcons() {
  /**
   * Get the icon URL for a given className
   * @param className The Roblox class name (e.g., "Part", "Script")
   * @returns The icon URL (Roblox CDN or fallback)
   */
  const getIconUrl = (className: string): string => {
    // If this icon has failed before, return fallback immediately
    if (failedIcons.has(className)) {
      return explorerPlaceholder;
    }

    return `${ROBLOX_ICON_BASE_URL}/${className}-Dark.webp`;
  };

  /**
   * Get the fallback icon URL
   */
  const getFallbackUrl = (): string => {
    return explorerPlaceholder;
  };

  /**
   * Mark an icon as failed (for caching)
   * @param className The class name that failed to load
   */
  const markIconFailed = (className: string): void => {
    failedIcons.add(className);
  };

  /**
   * Clear the failed icons cache
   */
  const clearFailedCache = (): void => {
    failedIcons.clear();
  };

  return {
    getIconUrl,
    getFallbackUrl,
    markIconFailed,
    clearFailedCache,
  };
}
