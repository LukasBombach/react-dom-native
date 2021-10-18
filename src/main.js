setTimeout(() => {
  console.log("1, 2");
}, 200);

console.log("test");

Deno.core.opSync("open_window");
