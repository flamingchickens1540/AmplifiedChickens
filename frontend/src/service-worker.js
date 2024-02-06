// @ts-nocheck
self.addEventListener('push', (event) => {
  console.log('Received a push message', event);
  let pushData = event.data.json();

  self.registration.showNotification(pushData.title, pushData)
      .then(() => {
        console.log("Notification shown")
      });
});

self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  clients.openWindow(event.notification.data.url)
      .then(() => {
          console.log("Notification clicked")
      });
});