console.log("hello javascript");

let windowId = null;

setInterval(() => {
  if (windowId === null) {
    windowId = Deno.core.opSync("create_window");
  } else {
    Deno.core.opSync("remove_window", windowId);
    windowId = null;
  }
}, 1000);
