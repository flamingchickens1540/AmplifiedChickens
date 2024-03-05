import { redirect } from '@sveltejs/kit';
import { count } from '$lib/stores';
import type { PageServerLoad } from './$types';
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT;

export const load = (async ({ cookies }) => {

    let accessToken = cookies.get("access_token")
    let scout_id = cookies.get("scout_id")

let match_res = await fetch(`${BACKEND_URL}/scout/get/current_match`)

    let match_key = await match_res.json()

    let res = await fetch(`${BACKEND_URL}/scout/request_team`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": accessToken
        } as HeadersInit,
    })

    if (res.status == 200) {
        var team_key = await res.json()
        console.log("team_key", team_key)
    } else if (res.status == 204) {
        count.set(1)
        redirect(303, "/app/match")
    } else {
        console.log("Error, you are not authorized to be here, please contact an admin.")
	redirect(303, "/")
    }

    return { team_key, scout_id, match_key };
}) satisfies PageServerLoad;
