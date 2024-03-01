import type { PageServerLoad } from './$types';

const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT;

export const load = (async ({ cookies }) => {
    const access_token = cookies.get('access_token')

    console.log("ADMIN")

    return {
        access_token,
    }
}) satisfies PageServerLoad;
