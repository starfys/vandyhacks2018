function asyncHarness(asyncHandler) {
  return event => {
    const promise = asyncHandler(event);
    event.waitUntil(promise);
  };
}

self.addEventListener("push", asyncHarness(async event => {
  const data = await event.data.json();
  console.log("Data recieved in push notification:", data);
  await self.registration.showNotification("ServiceWorker Cookbook", {
    body: "Push Notification Subscription Management"
  });
}));
