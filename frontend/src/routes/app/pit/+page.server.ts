import type { PageServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load = (async ({ cookies }) => {
	redirect("302", '/app/home')
    const scout_id = cookies.get('scout_id');
	const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_SVELTEKIT

    let res = await fetch(`${BACKEND_URL}/scout/get/unpitted`)

    console.log(res)

    let unpittscouted_teams = await res.json()

    return {
        scout_id,
	unpittscouted_teams
    };
}) satisfies PageServerLoad;
