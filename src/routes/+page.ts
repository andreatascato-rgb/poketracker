import { redirect } from "@sveltejs/kit";

/** Reindirizza / alla sezione Allenatore (landing quando si apre l'app). */
export function load() {
  redirect(302, "/profilo");
}
