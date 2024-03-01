import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;
    let accessToken = cookies.get("access_token")

    console.log("access token match: ", accessToken)

    let res = await fetch(`${BACKEND_URL}/scout/get/current_match`)

    let match_key = await res.json()
    console.log(match_key)

    return { accessToken, match_key };
}) satisfies PageServerLoad;
