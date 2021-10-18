setTimeout(() => {
  console.log("1, 2");
}, 200);

console.log("test");

console.log("windowId", Deno.core.opSync("open_window"));
console.log("windowId", Deno.core.opSync("open_window"));
console.log("windowId", Deno.core.opSync("open_window"));
