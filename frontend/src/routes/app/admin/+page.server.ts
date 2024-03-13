import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT;
    let access_token = cookies.get("access_token")

    let match_res = await fetch(`${BACKEND_URL}/scout/get/current_match`)
    let match_key = await match_res.json()

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

    let scout_data = sort(await res.json())

    return {
        access_token,
        match_key,
        scout_data
    }
}) satisfies PageServerLoad;

function sort(data: (string | number)[][]): (string | number)[][] {
    return data.sort((a, b) => {
        return (b[1] as number) - (a[1] as number)
    })
}
