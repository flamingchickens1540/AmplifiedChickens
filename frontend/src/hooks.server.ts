import { json, type Handle } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {


    console.log(event.cookies.getAll());
    console.log(event.url.pathname)

    if (event.url.pathname.startsWith('/app/admin')) {
        console.log("Checking admin auth")

        const is_admin = event.cookies.get('is_admin')

        console.log("is_admin: " + is_admin)

        let auth_res = await fetch("https://localhost:3007/auth/check")

        if (is_admin == undefined) {
            console.log("Unauthorized Request")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }

        if (auth_res.status == 200) {
            return await resolve(event)
        }

        return json({ status: 401, body: 'Unauthorized' })
    }


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

        let auth_res = await fetch("https://localhost:3007/auth/check", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ code: access_token, is_admin: true }),
        });

        if (auth_res.status != 200) {
            console.log("Unauthorized Request")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }
    }
    return await resolve(event)
}


