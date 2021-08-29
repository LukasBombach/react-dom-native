function run() {
  console.log("running the app");
}

function openWindow() {
  // todo opAsync
  return Deno.core.opSync("open_window");
}

function closeWindow() {
  console.log("closeWindow");
}

const ReactNativeSkia = { run };

export default ReactNativeSkia;
