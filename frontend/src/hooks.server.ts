import { type Handle, redirect, json } from '@sveltejs/kit'

export const handle: Handle = async ({ event, resolve }) => {

    console.log(event.cookies.getAll());
    console.log(event.url.pathname)

    const access_code = event.cookies.get('access_code')

    if (access_code == undefined) {
        console.log("Unauthorized Request")
        return json({
            status: 401,
            body: 'Unauthorized'
        });
    }

    if (event.url.pathname.startsWith('/app/admin')) {
        console.log("Checking admin auth")

        const is_admin = event.cookies.get('is_admin')

        console.log("is_admin: " + is_admin)

        let auth_res = await fetch("https://localhost:3007/auth/check", {
            method: "POST",
            headers: {
                Accept: "application/json",
                "Content-Type": "application/json",
            }, 
            body: JSON.stringify({ code: access_code, is_admin: true }),
        })

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

    let auth_res = await fetch("https://localhost:3007/auth/check", {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ code: access_code, is_admin: false }),
    })

    if (event.url.pathname.startsWith('/app')) {
        console.log("Checking auth")

        console.log(access_code)

        if (auth_res.status != 200) {
            console.log("Unauthorized Request")
            return json({
                status: 401,
                body: 'Unauthorized'
            });
        }
    }

    if (event.url.pathname == "/" && auth_res.status == 200) throw redirect(302, "/app/home");

    return await resolve(event)
}


