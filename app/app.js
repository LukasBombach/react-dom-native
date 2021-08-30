export function run() {
  console.log("running the app");
}

export function openWindow() {
  // todo opAsync
  return Deno.core.opSync("open_window");
}

export function closeWindow() {
  console.log("closeWindow");
}
