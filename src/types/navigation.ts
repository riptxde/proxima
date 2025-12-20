export type PageType =
  | "editor"
  | "script-hub"
  | "explorer"
  | "remote-spy"
  | "logs"
  | "settings";

export interface NavigationState {
  activePage: PageType;
}
