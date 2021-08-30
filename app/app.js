export function run() {
  console.log("running the app");
}

export function openWindow() {
  return Deno.core.opSync("open_window"); // todo opAsync
}

export function closeWindow() {
  console.log("closeWindow");
}
