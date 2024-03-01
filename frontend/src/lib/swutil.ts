const BACKEND_URL = import.meta.env.VITE_BACKEND_URL_FOR_FRONTEND;

export async function subscribeToPush(access_token: string | undefined) {
  console.log("ACCESS TOKEN: ", access_token);
  if (!access_token) return;

  const VAPID_PUBLIC_KEY = await fetchVapidKeys();

  let swRegistration = await navigator.serviceWorker.getRegistration();
  let pushManager = swRegistration?.pushManager;

  let subscriptionOptions = {
    userVisibleOnly: true,
    applicationServerKey: base64UrlToUint8Array(VAPID_PUBLIC_KEY),
  };
  try {
    let subscription = await pushManager?.subscribe(subscriptionOptions);

    console.log(subscription)

    localStorage.setItem("subscription", JSON.stringify(subscription));

    await fetch(`${BACKEND_URL}/register`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "x-access-token": access_token,
      },
      body: JSON.stringify(subscription),
    });
  } catch (error) {

    console.log(error);
  }
}

async function fetchVapidKeys() {
  return fetch(`${BACKEND_URL}/vapid`).then((resp) => resp.json().then(data => data.public_key));
}

function base64UrlToUint8Array(base64UrlData: string) {
  const padding = '='.repeat((4 - base64UrlData.length % 4) % 4);
  const base64 = (base64UrlData + padding)
    .replace(/\-/g, '+')
    .replace(/_/g, '/');

  const rawData = window.atob(base64);
  const buffer = new Uint8Array(rawData.length);

  for (let i = 0; i < rawData.length; ++i) {
    buffer[i] = rawData.charCodeAt(i);
  }
  return buffer;
}