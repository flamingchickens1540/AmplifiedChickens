import { redirect } from '@sveltejs/kit';
import { count } from '$lib/stores';
import type { PageServerLoad } from './$types';
import type { TeamMatchData } from '$lib/types';
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT;

export const load = (async ({ cookies, params }) => {

    let color = params.color

    if (color == "reload") {
        console.log("reload detected")
        let reload = true
        return { reload }
    }

    let accessToken = cookies.get("access_token")
    let scout_id = cookies.get("scout_id")

    let match_res = await fetch(`${BACKEND_URL}/scout/get/current_match`)

    let match_key = await match_res.json()

    let res = await fetch(`${BACKEND_URL}/scout/request_team`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": accessToken,
            "requested_color": color
        } as HeadersInit,
    })

    if (res.status == 200) {
        let team_data = await res.json()
        let team_key = team_data.team_key
        let team_color = team_data.color
        console.log("received team info: ", team_data)
        let reload = false
        return { team_key, team_color, scout_id, match_key, reload };
    } else if (res.status == 204) {
        redirect(303, "/app/match")
    } else {
        alert("Error, you are not authorized to be here, please contact an admin.")
        redirect(303, "/")
    }
}) satisfies PageServerLoad;
