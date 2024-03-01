import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;

export const load = (async ({ cookies }) => {

    let accessToken = cookies.get("access_token")

    console.log("access token scout: ", accessToken)

    let res = await fetch(`${BACKEND_URL}/scout/request_team`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": accessToken
        } as HeadersInit,
    })

    console.log(res)

    if (res.ok) {
        let team_key = await res.json()
        console.log("team_key", team_key)
    } else if (res.status == 204) {
        alert("No teams available")
        redirect(303, "/app/match")
    } else {
        alert("Error, you are not authorized to be here, please contact an admin.")
    }

    let data = await res.json();

    console.log("Team to be scouted: ", data)
    return { data };
}) satisfies PageServerLoad;
