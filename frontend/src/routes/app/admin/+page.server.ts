import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const access_token = cookies.get('access_token')
    
    console.log("ADMIN") 

    const queued_team_res = await fetch("http://localhost:3021/admin/users/get/queued", {
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
