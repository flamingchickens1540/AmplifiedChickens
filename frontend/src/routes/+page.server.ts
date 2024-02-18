import type { PageServerLoad } from './$types';



export const load = (async ({ cookies }) => {
    const a_code = cookies.get('access_token');

    const publicRedirectUrl = import.meta.env.VITE_PUBLIC_REDIRECT_URL;
    console.log('publicRedirectUrl', publicRedirectUrl);

    return {
        a_code
    };
}) satisfies PageServerLoad;