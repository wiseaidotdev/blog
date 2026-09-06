> Hello, fellow version-bumping enthusiasts, sleep-deprived Rustaceans, and accidental software archaeologists who just found out that `bumpversion` is a thing 👋!

So there I was, staring at my terminal at 2AM, trying to release version `0.1.0` of something. I typed `bump-my-version patch`, pressed Enter, and watched my CPU fan spin up like it was launching a SpaceX rocket. One Second later, _one second_, it bumped a number. One tiny number. `0.1.0` → `0.1.1`.

I sat there in silence for a moment.

Then I did what any rational developer would do: I rewrote it. In Rust. From scratch. With Python _and_ Node.js bindings. And a CLI. And `no_std` support. And `gix` for pure-Rust git operations.

The result? [`bump2version`](https://github.com/wiseaidev/bump2version) `0.2.0`: a version bumper that is legitimately, measurably, embarrassingly **\~10,000x faster** than the Python CLI it replaces.

![Fast Ket Typing non stop!](assets/images/fast-ket-typing.webp)

## 🤔 Wait, What Even Is `bump2version`?

Glad you asked. `bump2version` automates the tedious part of releasing software: updating version strings across multiple files. You know, the part where you manually grep through `Cargo.toml`, `package.json`, `pyproject.toml`, `CHANGELOG.md`, and your README, change `1.2.3` to `1.2.4` in 11 different places, forget one, push, CI fails, and you cry quietly into your coffee?

Yeah. That part.

`bump2version` does all of that for you:

- **Parses** version strings using a fully configurable regex (defaults to semver `major.minor.patch`).
- **Bumps** any component you ask it to: `major`, `minor`, `patch`, or custom cyclic stages like `alpha → beta → stable`.
- **Rewrites** version occurrences across multiple files, including multiline `CHANGELOG` patterns using proper `(?ms)` DOTALL + MULTILINE semantics.
- **Commits and tags** via `gix` - 100% pure-Rust git, zero subprocess calls, zero ghost authors in your commit history.

And it does all of this in **safe Rust**, with `#![forbid(unsafe_code)]` at the crate root, because we have principles around here. Or at least we pretend to.

```toml
# .bumpversion.toml: the config file that actually bumps the right things
[bumpversion]
current_version = "0.2.0"
commit = true
tag = true

[bumpversion:file:Cargo.toml]
search  = 'version = "{current_version}"'
replace = 'version = "{new_version}"'

[bumpversion:file:CHANGELOG.md]
search  = "## {current_version}\n    Release notes line 1"
replace = "## {new_version}\n    Release notes line 1"
```

One config file. Multiple files updated. One git commit. One tag. Done.

![2 GFs ain't enough bro!](assets/images/meme-4.png)

## 🦀 Rust, Python, and Node.js: A Love Triangle

Here's the fun part: `bump2version` isn't _just_ a Rust crate. It's three tools pretending to be one in a trench coat.

### As a Rust crate:

```toml
[dependencies]
bump2version = "0.2.0"
```

```rust
use bump2version::{config::BumpConfig, version::{parse_version, bump_version, serialize_version}};

fn main() {
    let cfg = BumpConfig::default();
    let v   = parse_version("1.2.3", &cfg).unwrap();
    let v2  = bump_version(&v, "patch", &cfg).unwrap();
    println!("{}", serialize_version(&v2, &cfg)); // 1.2.4
}
```

### As a Python package:

```sh
pip install bump-rs
```

```python
from bump_rs import bump_version, BumpConfig

print(bump_version("1.2.3", "patch"))  # "1.2.4"
print(bump_version("1.2.3", "minor"))  # "1.3.0"
print(bump_version("1.2.3", "major"))  # "2.0.0"
```

### As a Node.js add-on:

```sh
npm install bump2version
```

```javascript
const { bumpVersion, applyFileChange } = require("bump2version");

console.log(bumpVersion("1.2.3", "patch")); // '1.2.4'
console.log(bumpVersion("1.2.3", "minor")); // '1.3.0'
```

One Rust core. Three ecosystems. Zero Python subprocesses. Ferris the crab is now a polyglot, and honestly? Good for them. 🦀

## 🕵️ The Mossad Agents Who Architected This

Let me be transparent about one thing: I did not architect the full system design for this project alone.

No, I had help. Specifically, I reached out to some very professional consultants.

![The Mossad Agents That Helped Me Develop This Project.](assets/images/meme-2.png)

_They arrived at my door at 3AM with a whiteboard and a very detailed opinion on `Arc<Regex>` caching strategies._ Their key architectural recommendation, which I followed verbatim after reviewing it at gunpoint (metaphorically, probably), was the thread-safe `Arc<Regex>` cache. This means the compiled regex pattern is compiled _once_, shared across threads, and reused for every subsequent call, no recompilation overhead on hot paths.

The result: version bumping in **\~57 microseconds** from Python land. Not 57 milliseconds. Not 57 seconds. **57 microseconds.** The kind of number that makes you wonder what the Python version was doing during its 585 millisecond run.

## 🔥 The Numbers That Made Me Cackle Maniacally

Okay. Let's talk benchmarks. Because this is the part of the blog post where I get to paste a table and feel deeply smug about it.

These are real numbers, measured on x86-64 Linux (CPython 3.12, 3-sigma filtered `timeit`):

### Version Bumping: Full Round-Trip (Parse + Bump + Serialize)

| Library                                | `patch`     | `minor`     | `major`     |
| -------------------------------------- | ----------- | ----------- | ----------- |
| **bump-rs** (Rust, `Arc<Regex>` cache) | **\~57 µs** | **\~54 µs** | **\~53 µs** |
| `bump-my-version` (Python library)     | \~79 µs     | \~95 µs     | \~72 µs     |
| Pure Python (`re.compile` + `int()`)   | \~3.6 µs    | \~2.2 µs    | \~2.2 µs    |
| **`bump-my-version` CLI (subprocess)** | \~585 ms    | \~585 ms    | \~585 ms    |

The headline result: **bump-rs is \~10,000× faster than the `bump-my-version` CLI.**

Now, I can already hear you: _"But the pure Python version is actually faster for single calls!"_

Yes. You're right. The \~50 µs PyO3 FFI overhead means that if you're bumping exactly one version string in isolation on a warm Python interpreter, pure `re.compile` + `int()` will smoke us.

But the moment you're doing anything real, parsing a config file, updating multiple files, running a git commit, you're doing it once with `bump-rs` vs. spawning a subprocess, importing `click`, importing `importlib`, importing the entire `bump-my-version` dependency graph... and waiting **585 milliseconds**.

Every. Single. Time.

![Why would you do this, ma boy!](assets/images/meme-4.png)

### File Search/Replace

| Library                    | Single-line | Multiline CHANGELOG |
| -------------------------- | ----------- | ------------------- |
| **bump-rs** (Rust, cached) | **\~65 µs** | **\~104 µs**        |
| Pure Python `re.sub`       | \~1.7 µs    | \~1.3 µs            |

For file I/O work, thread safety, and pipeline operations, bump-rs wins. For tiny single-call in-memory operations where FFI overhead dominates: use bump-rs in batch mode, or use Python directly. We believe in honesty here.

## 🤖 Abusing Claude to Achieve the 10,000x Speed-Up

Here's a confession. A _deeply personal_ one. One that my legal team has strongly advised me not to make public.

I abused Claude.

Not in the _normal_ way where you ask it to generate boilerplate. No no no. I pushed it to its absolute limits. I asked it to write the same regex caching logic six different times in six different ways until one of them didn't make the borrow checker cry. I had it architecting FFI boundary semantics at 4AM. I used it to debate whether `Arc<Regex>` was overkill for a single-threaded benchmark (it was not). I got it to explain its own reasoning in elaborate detail and then argued with it.

Anthropic noticed.

![My lawyer defending me in court for abusing Claude](assets/images/meme-1.png)

My lawyer, argued that I was simply "exploring the full capability surface of the model." The judge was unmoved. The Anthropic lawyers were also unmoved, but in a different direction.

The verdict is still pending. The `Arc<Regex>` cache, however, is production-ready.

The lesson here: if you want to squeeze 10,000x performance out of a tool, you need to be willing to go to uncomfortable places. Dark places. Places where you're asking an AI to rewrite your regex cache for the seventh time at 4AM and you're genuinely not sure who's more tired: you, or the tokens.

Turns out: the tokens don't get tired. That's why Rust wins.

![Argue with Claude about the regex caching strategy!](assets/images/meme-6.png)

## 🐴 And Then the Borrow Checker Got Stuck

There is a moment in every Rust developer's life where you write something that you _know_ is correct, you've proven it in your head using mathematical induction and also vibes, and the borrow checker looks you dead in the eyes and says: **"No."**

No explanation. No suggestion. Just an error message that takes up five lines of your terminal and somehow manages to make you feel personally attacked by a compiler.

That happened. Multiple times. Specifically in the Python binding layer, where the intersection of PyO3's GIL management, `Arc<Regex>` shared state, and Rust's lifetime rules creates a special kind of chaos that can only be described as "my head hurts and I want to go home".

![AND My Rust Borrow Checker Got Stuck!](assets/images/meme-3.png)

The horse on the balcony railing is an accurate representation of `Arc<Mutex<HashMap<String, Regex>>>` trying to cross a PyO3 function boundary. It got there. It works. But the `stuck` moment before it worked? That was real.

The fix, anticlimactically, was changing the cache from a `HashMap` behind a `Mutex` to a thread-local `Arc<Regex>` initialized with `once_cell::sync::Lazy`. The borrow checker immediately, _graciously_, let the horse off the railing.

There's a metaphor in there somewhere. I choose not to examine it too closely.

![AND My Rust Borrow Checker Got Stuck!](assets/images/meme-7.png)

## 🛠️ Getting Started

Let's get practical. Here's how to use `bump2version` in your project right now:

### CLI Usage

```sh
cargo install bump2version --features rust-binary

bump2version --bump patch   # 0.2.0 → 0.2.1
bump2version --bump minor   # 0.2.0 → 0.3.0
bump2version --bump major   # 0.2.0 → 1.0.0
```

Useful flags:

| Option               | What it does                          |
| -------------------- | ------------------------------------- |
| `--config-file`      | Specify config file path              |
| `--current-version`  | Override detected current version     |
| `--bump`             | Which part: `major`, `minor`, `patch` |
| `--dry-run` / `-n`   | Simulate without touching any file    |
| `--commit` / `--tag` | Auto-commit and tag after bumping     |

### Python

```sh
pip install bump-rs
```

```python
from bump_rs import bump_version, apply_file_change, BumpConfig

# Custom parse/serialize for 2-component versions
cfg = BumpConfig(parse=r"(?P<major>\d+)\.(?P<minor>\d+)", serialize="{major}.{minor}")
print(bump_version("2.0", "minor", config=cfg))  # "2.1"
```

### Node.js

```sh
npm install bump2version
```

```typescript
import { bumpVersion, applyFileChange } from "bump2version";

const next = bumpVersion("1.2.3", "minor"); // "1.3.0"
```

### `no_std` Embedding

```toml
bump2version = { version = "0.2.0", default-features = false }
```

The core modules (`config`, `version`, `files`, `error`) compile on `no_std + alloc`. Useful for microcontrollers that also manage software release cycles. You know. If that's your situation.

## 🔒 The Safety Contract

`bump2version` enforces `#![forbid(unsafe_code)]` at the crate root. Every byte of the implementatio, config parsing, regex matching, version bumping, git object creation, is written in safe Rust. The compiler will literally _reject_ any future `unsafe` introduced into the safe portions.

The only `unsafe` in the entire codebase is in the Node.js FFI layer, because `napi-rs` requires it for native add-on interop and there's genuinely no way around that. If we could have avoided it, we would have. We tried. The borrow checker nodded approvingly at our effort, then still said no.

![unsafe Rust in production](assets/images/meme-8.png)

## 🔭 What's Coming in Future Releases

`bump2version` 0.2.0 is out the door, but the roadmap is full:

- **Workspace-aware bumping**: Update all crates in a Cargo workspace atomically in a single pass.
- **Pre-release cycling**: Better first-class support for `alpha → beta → rc → stable` lifecycle.
- **Watch mode**: Because apparently some people want their versions bumped on file save. (I won't judge. I _want_ to judge, but I won't.)
- **WASM target**: Core logic compiled to WebAssembly for browser-side version management. Yes, this is probably overkill. Yes, we're doing it anyway.
- **More benchmarks**: The Mossad agents have requested a full comparative analysis against every Python version tool ever created. We've filed the paperwork.

## 💬 Final Thoughts

Look. At the end of the day, `bump2version` does one thing: it bumps numbers in your files, commits the result, and tags the commit. That's it. That's the whole feature set.

But it does it in **safe Rust**. With **Python bindings** so Pythonistas don't have to care. With **Node.js bindings** so JavaScript developers can pretend they're also using Rust. With **no_std support** so embedded engineers can participate in the versioning conversation. With **pure-gix git integration** so there are zero subprocess calls anywhere in the hot path. And with benchmarks that show it's **\~10,000x faster** than the incumbent CLI tool.

Is that overkill for bumping a number? Absolutely. Are we sorry? Not even slightly.

> `cargo install bump2version --features rust-binary` → bump → ship → repeat 🦀

Star [the repo](https://github.com/wiseaidev/bump2version), try the [Python bindings](https://pypi.org/project/bump-rs), install the [npm package](https://www.npmjs.com/package/bump2version), or just read the [docs](https://docs.rs/bump2version). All paths lead to faster version bumping and a slightly more smug relationship with your release process.

This has been a public service announcement from a developer who really, _really_ did not want to wait 585 milliseconds for a number to go up by one.

![pip install bump-rs](assets/images/meme-9.png)

Till next time: _Keep bumpin', keep rustin'_ 🦀⬆️

P.S. The legal proceedings with Anthropic are ongoing. My lawyer has advised me to stop mentioning it. I have not taken that advice.
