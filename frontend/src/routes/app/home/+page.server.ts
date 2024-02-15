import type { PageServerLoad } from './$types';

export const load = (async ({ cookies }) => {
    const scout_name = cookies.get('scout_name');
    const scout_id = cookies.get('scout_id');
    const current_event_key = cookies.get('event_key');

    return {
        scout_name,
        scout_id,
        current_event_key
    };
}) satisfies PageServerLoad;