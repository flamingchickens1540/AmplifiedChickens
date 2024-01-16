// TODO: protect these endpoints

import type { Handle } from '@sveltejs/kit';
import 

export const handle: Handle = async ({ event, resolve }) => {
    if (event.url.pathname.startsWith('/admin')) {
        const user_id = event.cookies.get('user_id');
        const response = fetch('')
    }

}
