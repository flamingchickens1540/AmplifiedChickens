import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    let accessToken = cookies.get("access_token")

    console.log("access token match: ", accessToken)

    return { accessToken };
}) satisfies PageServerLoad;
