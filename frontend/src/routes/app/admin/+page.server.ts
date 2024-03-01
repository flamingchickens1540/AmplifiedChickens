import type { PageServerLoad } from './$types';

const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

export const load = (async ({ cookies }) => {
    const access_token = cookies.get('access_token')

    console.log("ADMIN")

    const queued_team_res = await fetch(`${BACKEND_URL}/admin/users/get/queued`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
            "x-access-token": access_token ?? ""
        }
    })

    const queued_scouts = await queued_team_res.json()

    console.log("queued_res: ", queued_scouts)
    return {
        access_token,
        queued_scouts
    }
}) satisfies PageServerLoad;
