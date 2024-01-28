import { json, type Handle } from '@sveltejs/kit'
export const handle: Handle = async ({ event, resolve }) => {
    console.log("HOOK!")
    const { cookies } = event
    const accessToken = String(cookies.get('sid'))

    if (event.url.pathname.startsWith('/app')) {
        // before endpoint or page is called
        let response = await fetch("https://localhost:3007/userAuth", {
            method: "POST",
            mode: "cors",
            cache: "no-cache",
            credentials: "same-origin",
            headers: {
                "Content-Type": "application/json",
            },
            redirect: "follow",
            referrerPolicy: "no-referrer",
            body: JSON.stringify({
                accessToken: accessToken,
            })
        })
        console.log(response);
        return response // FIXME: Return custom after checking response
    }

    const response = await resolve(event);
    return response;
}
