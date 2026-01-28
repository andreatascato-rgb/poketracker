import { redirect } from "@sveltejs/kit";

/** Reindirizza /archivio alla sottosezione Errori (prima sottosezione). */
export function load() {
  redirect(302, "/archivio/errori");
}
