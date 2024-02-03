import { json, type Handle } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {
    console.log(event.cookies.getAll());
    console.log(event.url.pathname)
    if (event.url.pathname.startsWith('/app')) {
        console.log("Checking auth")

        const access_token = event.cookies.get('access_code');

        console.log(access_token)

        if (access_token == undefined) {
            console.log("Unauthorized Request")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }
    }

    if (event.url.pathname.startsWith('/app/admin')) {
        console.log("Checking admin auth")

        const is_admin = event.cookies.get('is_admin')

        console.log("is_admin: " + is_admin)

        if (is_admin == undefined) {
            console.log("Unauthorized Request")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }
    }

    return await resolve(event)
} 
