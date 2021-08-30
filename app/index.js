import { openWindow } from "./app.js";

console.log("setTimeout 2000");
setTimeout(() => {
  console.log("opening a window");
  openWindow();

  console.log("setTimeout 2000");
  setTimeout(() => {
    console.log("timeout ended");
  }, 2000);
}, 2000);
