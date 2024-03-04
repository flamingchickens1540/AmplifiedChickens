import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const scout_name = cookies.get('scout_name');
    const scout_id = cookies.get('scout_id');
    const current_event_key = cookies.get('event_key');
    const access_token = cookies.get('access_token');
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT
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

    let scout_data = sort(scouts_data)

    return {
        scout_name,
        scout_id,
        current_event_key,
        access_token,
        scout_data
    };
}) satisfies PageServerLoad;

function sort(data: {name: string, percent: number}[]) {
    return data.sort((a, b) => {
        return a.percent - b.percent
    })
}
