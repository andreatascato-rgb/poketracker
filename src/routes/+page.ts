import { redirect } from "@sveltejs/kit";

/** Reindirizza / alla dashboard Allenatore (prima sezione attiva). */
export function load() {
  redirect(302, "/profilo/dashboard");
}
