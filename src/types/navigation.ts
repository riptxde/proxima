export type PageType =
  | "editor"
  | "script-hub"
  | "explorer"
  | "logs"
  | "settings";

export interface NavigationState {
  activePage: PageType;
}
