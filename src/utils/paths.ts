import { invoke } from "@tauri-apps/api/core";

export async function getScriptsPath(): Promise<string> {
  return await invoke<string>("get_scripts_path");
}
