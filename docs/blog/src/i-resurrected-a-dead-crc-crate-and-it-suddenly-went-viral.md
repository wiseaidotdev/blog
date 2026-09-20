> Bonjour 👋!

So there I was, browsing [crates.io](https://crates.io) at an hour that most reasonable people would describe as "the middle of the night", doing what all emotionally stable developers do at that hour: auditing dead Rust crates for signs of life.

That's when I found [`crc32`](https://crates.io/crates/crc32).

Not `crc32fast`. Not `crc`. Not `crc32c`. The original. The stubby, ancient, `crc32` crate. Last published in 2015. No changelog. No CI. A `Cargo.toml` that predates most of the stable Rust syntax I use daily. And sitting there, abandoned and unloved.

People were using this thing. Were probably _shipping_ software with it in their dependency tree. They just didn't know how to reach the crate maintainer, who had apparently retired to a cabin in the woods with no internet access sometime around when Rust 1.0 was announced.

I stared at those download numbers for thirty seconds.

Then I did what any emotionally stable rustacean would do: I resurrected it. From the ashes. In modern Rust. With `no_std`. With Python bindings. With a build-time proc-macro codegen crate. With CI/CD, Debian packaging, RPM packaging, and a Ferris logo.

You know. Normal stuff.

![shipping Ferris to production](assets/images/meme-18.jpeg)

## Wait, What Even Is CRC-32?

CRC-32 stands for Cyclic Redundancy Check, 32-bit variant. It's the checksum algorithm that:

- **ZIP files** use to verify archive integrity.
- **Ethernet frames** use to detect bit errors in transmission.
- **FDDI, PKZIP, PNG** and approximately half the binary protocols ever invented also use.
- Every `zlib` function you've ever called calls internally when you weren't paying attention.

In other words, CRC-32 is _everywhere_. It's the duct tape of data integrity. It's been computing checksums since before most of today's developers were handing in their first homework assignments.

The original Rust `crc32` crate implemented the byte-at-a-time version, the classic, the grandaddy variant. Simple. Correct. Slow as a 56k modem by 2024 standards.

I didn't just port it. I rewrote the whole thing from the zlib source, added slicing-by-4, slicing-by-8, and slicing-by-16 table variants, pulled the table-generation itself into a build-time proc-macro subcrate, added a streaming `Digest`, a GF(2) matrix `combine` function, and shipped the whole thing as a `no_std` library with Python and Node.js bindings.

All in 100% safe Rust. With `#![forbid(unsafe_code)]` at the crate root.

Because anything worth resurrecting is worth resurrecting _properly_.

## The Architecture

Let me tell you what the original `crc32` crate looked like. It was, essentially:

```
src/lib.rs
src/crc32gen.rs
src/crc32gen_file.rs
```

That's it. 3 files. A lookup table, a loop, a bitwise XOR. Beautiful in its simplicity, like a stone hut in a field. Functional. Zero frills. Absolutely not capable of 1 GiB/s throughput.

Here's what `crc32-v2` looks like today:

```
crc32-v2/
├── crc32-codegen/             ← proc-macro subcrate: generates all 17 CRC tables at build time
│   └── src/lib.rs             ← build.rs calls crc32_codegen::run() → writes to $OUT_DIR
├── src/
│   ├── lib.rs                 ← minimal entry point: #![no_std], module declarations, Python FFI
│   ├── tables.rs              ← crc32() byte-at-a-time baseline
│   ├── byfour.rs              ← crc32_little / _8 / _16 / crc32_big
│   ├── combine.rs             ← crc32_combine() via GF(2) matrix squaring in O(log n)
│   ├── digest.rs              ← streaming Digest with update() / finalize() / digest()
│   └── python.rs              ← PyO3 bindings (gated on feature = "python" + "std")
├── tests/
│   └── byfour.rs              ← 40 Rust integration tests
└── python/
    └── crc32_rs/
        └── __init__.py        ← re-exports all symbols from the compiled .so
```

The stone hut has been converted into a, uh, _well-appointed facility_.

![Before and after the refactor](assets/images/meme-2.webp)

## The Magic Trick

This is my personal favourite part of the whole project. And possibly the most unhinged.

CRC-32 slicing-by-16 requires 17 lookup tables: one 256-entry big-endian table, and 16 256-entry little-endian tables at different offsets. That's 17 × 256 × 4 = 17,408 bytes of lookup table data that needs to exist at runtime.

The naive way: write the tables by hand and commit 17,000 bytes of constants to git as a `.rs` file.

The normal way: generate them at startup and cache them in a `static`.

The way I did it: proc-macro subcrate (`crc32-codegen`) that is invoked from `build.rs` at compile time, generates all 17 tables using the CRC-32 polynomial arithmetic, writes them to `$OUT_DIR/crc_tables.rs`, and the main crate `include!`s the output. Zero runtime initialization. Zero startup cost. The tables are baked into the binary at link time.

```toml
[build-dependencies]
crc32-codegen = { path = "crc32-codegen" }
```

```rust
fn main() {
    crc32_codegen::run();
}
```

That's the entire `build.rs`. One function call. The codegen crate does all the polynomial math, formats the Rust source, and writes it to disk. The main crate wakes up with all 17 tables pre-computed, pre-verified, and pre-formatted.

This is the kind of thing that inspires either awe or a mildly concerned Slack message from your coworkers. There is no middle ground.

![Ferris generating a lookup table](assets/images/meme-19.jpeg)

## The 4 Speeds of CRC-32

Let me paint you a picture of what "slicing-by-N" actually means, because it's one of those ideas that sounds arcane until you see the numbers and then you can't stop thinking about it.

The classic CRC-32 loop looks like this:

```rust
for &byte in buf {
    let index = (crc ^ u32::from(byte)) & 0xff;
    crc = CRC_TABLE[0][index as usize] ^ (crc >> 8);
}
```

One byte per iteration. One table lookup. On a 1 MiB payload that's 1,048,576 iterations. At ~2 ns per iteration (fast CPU, warm cache), that's ~2ms. Not terrible. Not great.

Slicing-by-4 says: what if we process _four_ bytes per iteration instead? You pre-compute four separate 256-entry tables (one for each byte offset), and in each step you XOR together four table lookups instead of one. Four bytes per iteration, four table lookups, but modern CPUs can do all four lookups in parallel because there are no data dependencies between them. Result: roughly 2.4× faster.

Slicing-by-8 does the same with eight tables. Slicing-by-16 pushes it to sixteen tables and sixteen bytes per step, letting the CPU's out-of-order execution and instruction-level parallelism do the heavy lifting. With `#[inline(always)]` on the inner fold functions, fat LTO, and `overflow-checks = false` in the bench profile, LLVM sees through all the abstraction and generates machine code that is, frankly, embarrassingly fast for code written entirely in safe Rust.

The full breakdown:

| Method                                |      1 B |      64 B |      1 KiB |        64 KiB |          1 MiB |       Throughput |
| ------------------------------------- | -------: | --------: | ---------: | ------------: | -------------: | ---------------: |
| `crc32` (byte-at-a-time)              |     2 ns |    167 ns |   2,856 ns |    188,134 ns |   2,916,562 ns |       ~343 MiB/s |
| `crc32_little` (slicing-by-4)         |     3 ns |     73 ns |   1,086 ns |     82,129 ns |   1,199,953 ns |       ~833 MiB/s |
| `crc32_little_8` (slicing-by-8)       |     3 ns |     61 ns |     922 ns |     56,486 ns |   1,004,746 ns |     ~1,004 MiB/s |
| **`crc32_little_16` (slicing-by-16)** | **3 ns** | **39 ns** | **753 ns** | **48,101 ns** | **781,535 ns** | **~1,282 MiB/s** |
| `crc32fast` (SIMD via `pclmulqdq`)    |    11 ns |     20 ns |     102 ns |      5,802 ns |      89,204 ns |    ~11,300 MiB/s |

`crc32_little_16` hits **~1,282 MiB/s** on a 1 MiB payload. That's **3.7× faster than the baseline**, without a single SIMD intrinsic, without a single `unsafe` block, and without any CPU feature detection. It Just Works on x86-64, ARM, RISC-V, WASM, and whatever exotic architecture you're running this week.

Note that tiny 1-byte payloads still favour the baseline `crc32` (2 ns vs 3 ns) because the slicing variants pay a small alignment prologue overhead. We document this honestly, unlike some people's README files.

![Surprised Ferris](assets/images/meme-20.jpeg)

## The Cross-Library Reality Check

Look. I know what you're thinking. _"But crc32fast uses SIMD and does 11 GiB/s. Doesn't that make this pointless?"_

No. Here's why.

`crc32fast` is genuinely excellent for x86-64 with `pclmulqdq`. But `crc32-v2` occupies a different niche:

| Library               | Method            |   Time (1 MiB) |       Throughput | Type                      |
| --------------------- | ----------------- | -------------: | ---------------: | ------------------------- |
| `crc32fast`           | `hash`            |      89,204 ns |    ~11,300 MiB/s | SIMD (`pclmulqdq`) / Rust |
| `zlib-rs`             | `crc32`           |      96,968 ns |    ~10,390 MiB/s | SIMD / Rust               |
| Python (`zlib.crc32`) | C extension       |     355,270 ns |     ~2,815 MiB/s | SIMD / C                  |
| **`crc32-v2`**        | `crc32_little_16` | **781,535 ns** | **~1,282 MiB/s** | **Pure safe Rust**        |
| Python (`crcmod`)     | C extension       |   2,865,836 ns |       ~349 MiB/s | Non-SIMD / C              |

`crc32fast`: amazing. `zlib-rs`: also amazing. Both use runtime SIMD dispatch and `unsafe`. Both don't compile to WASM without special handling. Both don't work in `no_std` environments without a careful dance.

`crc32-v2` compiles everywhere Rust compiles. Embedded RISC-V? Yes. `wasm32-unknown-unknown`? Yes. A microcontroller that needs to verify a firmware checksum before flashing? Yes. A kernel module? With `#![no_std]` and a custom allocator? _Also_ yes.

When you need 11 GiB/s you use `crc32fast`. When you need something that works anywhere, has zero dependencies outside `alloc`, and is produced by a compiler that will literally refuse to let you write unsafe code: you use `crc32-v2`. There are more situations in the second category than you'd expect.

![use crc32-v2 davai davai](assets/images/meme-21.jpeg)

## Python Bindings

Here's a section I am writing in the spirit of radical honesty. Shipping Python bindings for a Rust library requires configuring:

- `maturin` (the build tool)
- `pyo3` (the FFI framework)
- `pyproject.toml` (the Python packaging config)
- A `python/crc32_rs/__init__.py` that re-exports from the compiled `.so`
- The `module-name` field in `pyproject.toml` matching the `#[pymodule]` name in Rust

If any one of these four things is even slightly wrong, Python greets you with:

```python
ModuleNotFoundError: No module named 'crc32_rs'
```

Not a helpful error. Not `"the module-name in pyproject.toml doesn't match the pymodule attribute"`. Just: it doesn't exist. Good luck.

I went through approximately four rounds of this before getting it right. The root cause was a mismatch between `module-name = "crc32_v2._crc32_v2"` (incorrect) and `module-name = "crc32_rs._crc32_v2"` (correct). One underscore in the wrong namespace and Python acts as if the entire compiled extension simply does not exist in this universe.

The fix took three characters. The investigation took forty-five minutes.

```python
>>> from crc32_rs import crc32, crc32_little_16, crc32_bytes, crc32_hex, Digest
>>>
>>> print(hex(crc32(b"Hello, world!")))
0xebe6c6e6
>>> print(hex(crc32_little_16(b"Hello, world!")))
0xebe6c6e6
>>> print(crc32_bytes(b"Hello, world!").hex())
ebe6c6e6
>>> print(crc32_hex(b"Hello, world!"))
ebe6c6e6
>>>
>>> d = Digest()
>>> d.update(b"Hello, ")
>>> d.update(b"world!")
>>> print(hex(d.finalize()))
0xebe6c6e6
>>> print(repr(d))
Digest(crc=0xEBE6C6E6)
```

Now it works. The Python interpreter is stuffed. We do not speak of the forty-five minutes.

![same same](assets/images/meme-22.jpeg)

## Rust for Python Developers

Let's talk about something that the throughput benchmarks don't fully capture: what happens to performance when you're computing CRC-32 on _tiny_ payloads? Like, really tiny. 1 byte tiny.

The issue with Python's [`zlib.crc32`](https://docs.python.org/3/library/zlib.html#zlib.crc32) is not that it's slow for large inputs, it delegates to SIMD C code under the hood and handles 1 MiB payloads at 2,815 MiB/s, which is genuinely fast. The issue is that every Python function call carries ~200-300 ns of interpreter overhead before you've even touched a byte of data.

| Payload | Python `zlib.crc32` | Rust `crc32-v2` |   Speedup |
| ------- | ------------------: | --------------: | --------: |
| 1 B     |             ~297 ns |           ~2 ns | **~148×** |
| 64 B    |             ~301 ns |          ~39 ns | **~7.7×** |

148 times faster on a single byte. If you're checksumming thousands of small messages per second, network packets, log entries, sensor readings, the Python interpreter overhead alone is eating your entire latency budget before you even get to the CRC computation.

Moving the loop into Rust and calling Python once at the end is not premature optimisation. It is the correct architectural decision. `crc32-v2` makes that decision easier to implement.

## `no_std`: Ferris Goes Embedded

This was the part I underestimated most. Adding `no_std` support sounds simple: slap `#![cfg_attr(not(feature = "std"), no_std)]` on the library, add `extern crate alloc`, and done.

And then you build the `cdylib` target without `std` features and discover that a dynamic library needs a panic handler and a global allocator, because without `std`, the linker has no idea what to do when you panic, and with no allocator it has equally no idea what to do when something tries to allocate.

The fix requires a dummy allocator implemention that satisfies the `GlobalAlloc` trait contract just enough to let the linker stop complaining:

```toml
# Features that gate the standard library
[features]
default = ["std"]
std = []
python = ["pyo3", "std"]    # Python bindings require std
node = ["napi", "napi-derive", "napi-build", "std"]  # Node.js too
```

```rust
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
```

And `panic = "abort"` in `[profile.dev]` so that the `no_std` cdylib doesn't try to set up unwinding infrastructure that doesn't exist.

Result: `cargo build --no-default-features` completes successfully. Ferris is now embeddable. The microcontroller community rejoices.

## Safety

`#![forbid(unsafe_code)]` is enforced at the crate root. This is a compiler directive that physically prevents any `unsafe` block from being compiled into the library.

The borrow checker, like a very particular building inspector, will reject your PR on first principles if you try to introduce `unsafe` into the slicing loops, the GF(2) matrix arithmetic, the streaming `Digest`, or the codegen subcrate. You can argue with it. You can find workarounds. The borrow checker does not care. It has seen your workarounds and it finds them unconvincing.

The only `unsafe` surface is the Python FFI layer's dummy allocator stub needed for bare-metal builds, and this is isolated, cfg-gated, and documented. The rest of the crate is provably safe at compile time.

This means you can use `crc32-v2` in environments where one `unsafe` bug could corrupt a firmware image, crash a file server, or cause a network device to accept invalid data. The compiler provides the guarantee. Not the tests. Not the code review. The _compiler_.

## Getting Started

### Rust

```toml
[dependencies]
crc32-v2 = "0.2.0"

# For embedded / no_std:
# crc32-v2 = { version = "0.2.0", default-features = false }
```

```rust
use crc32_v2::{crc32, crc32_combine, byfour::crc32_little_16, Digest};

// One-shot
assert_eq!(crc32(0, b"123456789"), 0xCBF43926);

// Fastest pure-software path (~1,282 MiB/s)
assert_eq!(crc32_little_16(0, b"Hello, world!"), 0xEBE6_C6E6);

// Streaming
let mut d = Digest::new();
d.update(b"Hello, ");
d.update(b"world!");
assert_eq!(d.finalize(), 0xEBE6_C6E6);

// Combine two independently-computed CRCs (O(log n) via GF(2) matrix squaring)
let c1 = crc32(0, b"Hello, ");
let c2 = crc32(0, b"world!");
assert_eq!(crc32_combine(c1, c2, 6), crc32(0, b"Hello, world!"));
```

### Python

```sh
pip install crc32-rs
```

```python
>>> from crc32_rs import crc32, crc32_little_16, crc32_bytes, crc32_hex, Digest

>>> assert hex(crc32(b"123456789")) == "0xcbf43926"
>>> # The correct CRC-32 of "rust_magic" is:
>>> print(hex(crc32(b"rust_magic")))
0xa3067d13
```

### Node.js

```sh
npm install crc32-rs
```

```js
const { crc32, crc32Little16 } = require("crc32-rs");
console.log(crc32(Buffer.from("Hello, world!")).toString(16)); // ebe6c6e6
```

## What's Next

The crate is at `0.2.0` and functionally complete for its initial scope. The roadmap includes:

- **Slicing-by-32**: Because if 16 tables give us 3.7× speedup, we're obligated to find out where the speedup curve flattens.
- **`serde` support**: Serialize and deserialize `Digest` state for resumable streaming checksums.
- **WASM target**: The `no_std` foundation is already there. The WASM build should be trivial in theory. "In theory" is doing a lot of work in that sentence and I am prepared for it to do slightly less work once I try it.

## Closing Thoughts

`crc32-v2` started as a resurrection project. Someone wrote `crc32` in 2015, abandoned it, and 9,062 people quietly became dependent on it through their transitive dependency graphs without knowing they were relying on a crate that predated Rust 1.0.

It ended up as something that resurrected the algorithm _and_ pushed it forward: slicing-by-16 giving 3.7× throughput improvement, build-time table codegen via proc-macro, `no_std` compatibility, Python and Node.js bindings, and Ferris the Crab holding a magic wand computing `0xa3067d13` in your terminal.

The original `crc32` crate was doing its best with 2015-era Rust. We can do more. We should do more. The 9,062 people who downloaded it deserve more.

> `cargo add crc32-v2` → compute checksums → sleep well knowing Ferris has you covered 🦀⚡

Star [the repo](https://github.com/wiseaidev/crc32-v2), try the [Python bindings](https://pypi.org/project/crc32-rs), install the [npm package](https://www.npmjs.com/package/crc32-rs), or read the [docs](https://docs.rs/crc32-v2) if you still have questions.

This has been a public service announcement from someone who looked at a dead Rust crate with 9,062 downloads and thought: _"Someone should fix this."_

That someone was me. And honestly? It was a great use of my 2AM.

![same same](assets/images/meme-22.jpg)

Till next time: _Keep the bytes honest. Keep the checksums rolling._ 🦀🔄

P.S. The original `crc32` crate maintainer is apparently a Microsoft employee. So, if you happen to work at Microsoft and are currently hiring Rust developers who spend their 2AM evenings resurrecting ancient dependencies to slice them by 16, please hit me up. We can bond over polynomial arithmetic. Thanks for the help <3!
