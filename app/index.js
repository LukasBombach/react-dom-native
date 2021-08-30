import { openWindow } from "./app.js";

console.log("opening a window");
const rid = openWindow();
console.log("resource id is", rid);

console.log("setTimeout 2000");
setTimeout(() => {
  console.log("timeout ended");
}, 2000);
