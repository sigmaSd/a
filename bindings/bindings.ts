// Auto-generated with deno_bindgen
function encode(v: string | Uint8Array): Uint8Array {
  if (typeof v !== "string") return v
  return new TextEncoder().encode(v)
}

function decode(v: Uint8Array): string {
  return new TextDecoder().decode(v)
}

function readPointer(v: any): Uint8Array {
  const ptr = new Deno.UnsafePointerView(v as bigint)
  const lengthBe = new Uint8Array(4)
  const view = new DataView(lengthBe.buffer)
  ptr.copyInto(lengthBe, 0)
  const buf = new Uint8Array(view.getUint32(0))
  ptr.copyInto(buf, 4)
  return buf
}

const url = new URL(
  "/home/mrcool/dev/rust/cargo_target_dir/debug",
  import.meta.url,
)

let uri = url.pathname
if (!uri.endsWith("/")) uri += "/"

// https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibrarya#parameters
if (Deno.build.os === "windows") {
  uri = uri.replace(/\//g, "\\")
  // Remove leading slash
  if (uri.startsWith("\\")) {
    uri = uri.slice(1)
  }
}

const { symbols } = Deno.dlopen(
  {
    darwin: uri + "libx.dylib",
    windows: uri + "x.dll",
    linux: uri + "libx.so",
  }[Deno.build.os],
  {
    add: { parameters: ["buffer", "usize"], result: "i32", nonblocking: false },
  },
)
export type Input = {
  a: number
  b: number
}
export function add(a0: Input) {
  const a0_buf = encode(JSON.stringify(a0))

  let rawResult = symbols.add(a0_buf, a0_buf.byteLength)
  const result = rawResult
  return result
}
