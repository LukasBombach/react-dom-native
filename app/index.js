import { openWindow, closeWindow } from "./app.js";

console.log("window 1", openWindow());
console.log("window 2", openWindow());

let windowRid = openWindow();
console.log("opened", windowRid);

setInterval(() => {
  if (windowRid === null) {
    windowRid = openWindow();
    console.log("opened", windowRid);
  } else {
    windowRid = closeWindow(windowRid);
    console.log("closed", windowRid);
  }
}, 1000);
