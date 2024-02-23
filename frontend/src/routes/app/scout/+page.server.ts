import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {

    let accessToken = cookies.get("access_token")

    console.log("access token scout: ", accessToken)

    let res = await fetch("http://localhost:3021/scout/request_team", {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": accessToken ?? ""
        },
    })

    let data = await res.json();

    console.log("Team to be scouted: ", data)
    return { data };
}) satisfies PageServerLoad;
