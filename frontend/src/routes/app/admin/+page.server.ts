import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const access_token = cookies.get('access_token')
    
    console.log("ADMIN") // THIS TRIGGERS

    return {
        access_token
    }
}) satisfies PageServerLoad;