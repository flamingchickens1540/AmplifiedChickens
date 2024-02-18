import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {

    let accessToken = cookies.get("access_token");

    let res = await fetch("https://scout.team1540.org/api/scout/request_team", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": accessToken
        },
    })

    let data = await res.json();

    console.log("Team to be scouted: ", data)
    return { data };
}) satisfies PageServerLoad;