export async function subscribeToPush(access_token: string | undefined) {
  console.log("ACCESS TOKEN: ", access_token);
  if (!access_token) return;

  const VAPID_PUBLIC_KEY = await fetchVapidKeys();

  let swRegistration = await navigator.serviceWorker.getRegistration();
  let pushManager = swRegistration?.pushManager;

  let subscriptionOptions = {
    userVisibleOnly: true,
    applicationServerKey: urlBase64ToUint8Array(VAPID_PUBLIC_KEY)
  };
  try {
    let subscription = await pushManager?.subscribe(subscriptionOptions);

    console.log(subscription)

    localStorage.setItem("subscription", JSON.stringify(subscription));

    await fetch("https://localhost:3007/register", {
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
  return fetch("https://localhost:3007/vapid.json").then((resp) => resp.json().then(data => data.public_key));
}

function urlBase64ToUint8Array(base64String: String) {
  var padding = "=".repeat((4 - (base64String.length % 4)) % 4);
  var base64 = (base64String + padding).replace(/\-/g, "+").replace(/_/g, "/");

  var rawData = window.atob(base64);
  var outputArray = new Uint8Array(rawData.length);

  for (var i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }
  return outputArray;
}