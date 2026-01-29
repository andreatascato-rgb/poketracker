/**
 * Servizio export/backup: wrapper tipati per command Tauri (open_export_folder, set_export_dir, get_export_dir).
 */
import { invoke } from "$lib/services/tauri";

export async function getExportDir(): Promise<string> {
  return invoke<string>("get_export_dir");
}

export async function openExportFolder(): Promise<void> {
  return invoke("open_export_folder");
}

export async function setExportDir(path: string | null): Promise<void> {
  return invoke("set_export_dir", { path });
}
