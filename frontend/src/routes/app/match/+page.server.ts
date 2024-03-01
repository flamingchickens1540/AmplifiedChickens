import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT;
    let accessToken = cookies.get("access_token")

    console.log("access token match: ", accessToken)

    let res = await fetch(`${BACKEND_URL}/scout/get/current_match`)

    let match_key = await res.json()
    console.log("Match key", match_key)
let scouts_res = await fetch(`${BACKEND_URL}/admin/users/get/all`, {
            method: "GET",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            }
        })
        console.log(scouts_res)

        if (!scouts_res.ok) {
            console.error("Failed to fetch scout percents")
        }

	let scouts_data = await scouts_res.json()

    return { accessToken, match_key, scout_names: scouts_data[0], scout_percents: scouts_data[1] };
}) satisfies PageServerLoad;
