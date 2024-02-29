import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    let BACKEND_URL = import.meta.env.VITE_BACKEND_URL;
    let accessToken = cookies.get("access_token")
    let name = cookies.get("scout_name")

    console.log("access token match: ", accessToken)

    console.log("scout name match: ", name)

    let res = await fetch(`${BACKEND_URL}/match/get/current`)

    let current_match = await res.json();

    console.log(current_match)

    return { accessToken, name, current_match };
}) satisfies PageServerLoad;
