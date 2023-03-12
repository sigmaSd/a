import { $ } from "https://deno.land/x/dax@0.28.0/mod.ts";

await $`git clone https://github.com/helix-editor/helix`;
$.cd("helix");
await $`cargo b --release`;
