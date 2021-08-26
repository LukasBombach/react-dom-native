const windowArgs = {
  rid: 0,
  title: "javascript window",
  size: { width: 640, height: 480 },
  position: { x: 300, y: 300 },
};

let windowId = Deno.core.opSync("create_window", windowArgs);

setInterval(() => {
  if (windowId === null) {
    windowId = Deno.core.opSync("create_window", windowArgs);
  } else {
    Deno.core.opSync("remove_window", windowId);
    windowId = null;
  }
}, 1000);
