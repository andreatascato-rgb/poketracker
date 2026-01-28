import { redirect } from "@sveltejs/kit";

/** Reindirizza /impostazioni alla sottosezione Profili (default). */
export function load() {
  redirect(302, "/impostazioni/profili");
}
