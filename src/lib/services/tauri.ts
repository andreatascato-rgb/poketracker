/**
 * Re-export invoke da @tauri-apps/api/core per uso nei servizi.
 * Ogni servizio usa invoke<T>(command, payload) con tipi da lib/types/api.ts.
 */
export { invoke } from "@tauri-apps/api/core";
