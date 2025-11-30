export type PageType = "editor" | "script-hub" | "logs" | "settings";

export interface NavigationState {
  activePage: PageType;
}
