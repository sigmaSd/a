import { Pty } from "https://deno.land/x/deno_pty_ffi@0.6.0/mod.ts";

const pty = await Pty.create({
  cmd: "ls",
  args: [],
  env: [],
});

console.log(await pty.read());
