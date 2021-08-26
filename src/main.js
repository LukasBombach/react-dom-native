console.log("hello javascript");
console.log("window 1", Deno.core.opSync("create_window"));
console.log("window 2", Deno.core.opSync("create_window"));
console.log("window 3", Deno.core.opSync("create_window"));
