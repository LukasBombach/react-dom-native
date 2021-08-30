import { openWindow, closeWindow } from "./app.js";

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
