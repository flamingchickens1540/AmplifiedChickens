import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const a_code = cookies.get('access_token');

    return {
        a_code
    };
}) satisfies PageServerLoad;
