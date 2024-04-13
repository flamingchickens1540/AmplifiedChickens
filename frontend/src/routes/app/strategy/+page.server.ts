import type { PageServerLoad } from './$types';
export const load = (async ({ cookies }) => {
  const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT

  const scout_id = cookies.get('scout_id');

  let res = await fetch(`${BACKEND_URL}/admin/users/get/all`, {
      method: "GET",
      headers: {
          Accept: "application/json",
          "Content-Type": "application/json",
      }
  })

  if (!res.ok) {
      console.error("Failed to fetch scout percents")
  }

  let scouts_data = await res.json()

  let scout_data_sorted = sort(scouts_data)

  return scout_id
}) satisfies PageServerLoad;

function sort(data: (string | number)[][]): (string | number)[][] {
  return data.sort((a, b) => {
      return (b[1] as number) - (a[1] as number)
  })
}