import { type Handle, redirect } from "@sveltejs/kit"
const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

export const handle: Handle = async ({ event, resolve }) => {

    const access_token = event.cookies.get("access_token")

    if (access_token == undefined && event.url.pathname != "/" && !event.url.pathname.startsWith("/app/admin")) {
        return redirect(302, "/");
    }

    let isAdminPath = event.url.pathname.startsWith('/app/admin');

    let auth_res = await fetch(`${BACKEND_URL}/auth/check`, {
        method: "POST",
        headers: {
            Accept: "application/json",
            "Content-Type": "application/json",
        },
        body: JSON.stringify({ access_token: access_token, is_admin: isAdminPath }),
    });

    if (auth_res.status != 200 && event.url.pathname != "/") {
        return redirect(302, "/");
    }

    if (event.url.pathname == "/" && auth_res?.status == 200) {
        return redirect(302, "/app/home");
    }

    return await resolve(event);
}
