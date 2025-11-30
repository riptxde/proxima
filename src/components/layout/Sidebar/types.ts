import type { PageType } from "@/types/navigation";
import type { Component } from "vue";

export interface SidebarButton {
  id: PageType;
  label: string;
  icon: Component;
}
