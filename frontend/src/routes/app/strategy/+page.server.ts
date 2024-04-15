import type { PageServerLoad } from './$types';
export const load = (async ({ cookies }) => {
  const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT

  const scout_id = cookies.get('scout_id');

  return {scout_id}
}) satisfies PageServerLoad;
