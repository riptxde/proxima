export type PageType =
  | "editor"
  | "script-hub"
  | "explorer"
  | "remote-spy"
  | "logs"
  | "launcher"
  | "settings";

export interface NavigationState {
  activePage: PageType;
}
