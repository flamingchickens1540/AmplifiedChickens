import { type Handle, redirect, json } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {

	console.log(event.cookies.getAll());
	console.log(event.url.pathname)

    const access_token = event.cookies.get('access_token')
    // const scout_access_key = event.cookies.get('scout_access_key')

    let auth_res = undefined

    if (access_token == undefined) {
        if (event.url.pathname == "/") {
            return await resolve(event)
        }

        console.log("Unauthorized Request")
        return json({
            status: 401,
            body: 'Unauthorized'
        })
    }

    // if (event.url.pathname.startsWith('/app/scout')) {


    // }

    if (event.url.pathname.startsWith('/app/admin')) {
        console.log("Checking admin auth")

        auth_res = await fetch("https://localhost:3007/auth/check", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ access_token: access_token, is_admin: true }),
        })

        if (auth_res.status == 200) {
            return await resolve(event)
        }

        return json({ status: 401, body: 'Unauthorized Request: Admin' })
    } else if (event.url.pathname.startsWith('/app')) {

        console.log("Checking auth")

        auth_res = await fetch("https://localhost:3007/auth/check", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ access_token: access_token, is_admin: false }),
        })

        console.log(access_token)

        if (auth_res.status != 200) {
            console.log("Unauthorized Request: Scout")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }
    }

    console.log(auth_res)
    if (auth_res != undefined) {
        if (event.url.pathname == "/" && auth_res?.status == 200) throw redirect(302, "/app/home");
    }

    return await resolve(event)
}


