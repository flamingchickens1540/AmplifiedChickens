import { json, type Handle } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {
    console.log(event.cookies)
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

    return await resolve(event)
} 
