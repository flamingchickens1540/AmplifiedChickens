import { type Handle, redirect } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {
    const access_code = event.cookies.get('access_code');
    console.log(access_code)

    if (event.url.pathname == "/" && access_code) throw redirect(302, "/app/home");

    if (event.url.pathname != "/" && !access_code) throw redirect(302, "/");

    if (event.url.pathname.startsWith('/app/admin')) {
        if (event.cookies.get('is_admin') == undefined) { //TODO: secure endpoint properly with backend API
            throw redirect(302, "/")
        }
    }
    
    return await resolve(event)
} 