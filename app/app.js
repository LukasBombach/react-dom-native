export function run() {
  console.log("running the app");
}

export function openWindow() {
  return Deno.core.opSync("open_window"); // todo opAsync
}

export function closeWindow(rid) {
  return Deno.core.opSync("close_window", rid);
}
