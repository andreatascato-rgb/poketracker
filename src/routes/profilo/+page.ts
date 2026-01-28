import { redirect } from "@sveltejs/kit";

/** Reindirizza /profilo alla sottosezione Dashboard (default). */
export function load() {
  redirect(302, "/profilo/dashboard");
}
