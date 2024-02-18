import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const scout_id = cookies.get('scout_id');

    return {
        scout_id
    };
}) satisfies PageServerLoad;