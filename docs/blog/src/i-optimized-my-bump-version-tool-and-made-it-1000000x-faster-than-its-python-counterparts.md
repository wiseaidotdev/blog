> Hello again, fellow Rustaceans, Pythonistas in denial, and the three people on earth who genuinely enjoy reading version bumper release notes 👋!

If you were here for my last post, where I rewrote [`bump2version`](https://github.com/c4urself/bump2version) in Rust and declared it **~10,000x faster** than the Python CLI, you may recall that I ended it with a vague threat about future benchmarks. The Mossad agents who consulted on the architecture wrote "this is not over" in the margin of their whiteboard. I ignored it.

I should have listened.

Because in the weeks since that post, I went back in. _Deep in._ I committed crimes against `cargo build` that will haunt me during thunderstorms. I `--release`'d things that should not be `--release`'d. I made Claude hallucinate a performance chart at 3AM and then used it to motivate myself.

The result: [`bump2version 0.2.1`](https://github.com/wiseaidev/bump2version/releases/tag/v0.2.1).

> **Now 1,000,000× faster than its Python counterparts.** Yes, I am counting subprocess overhead.

![Ferris the crab is now a Ferris Soldier!](assets/images/soviet-benchmarks.png)

_The Soviet material resurfaced. It always does._

Let's get into it.

## 🤔 Wait, We Already Did This. Why Are We Here Again?

Because `0.2.0` shipped, and I immediately opened my Gmail inbox.

The audacity of the open-source community. The audacity, bro! I build something 10,000x faster and within _days_ there are requests. "Can it watch files?", "Can it detect which manifests I'm using?", "Can I run it in the browser?", "Can I use it with Go?", "Can I use it with Java?", "Can my Ruby project use it?". I ain't got no time for this. I need to take a little break and play CS2 with the boys!

I looked at these emails. I looked at the ceiling. I looked at my coffee. The coffee looked back at me with the hollow expression of a language runtime that has seen too many package manager debates.

And then I cracked my knuckles and got to work.

![This is fine](assets/images/this-is-fine-rust.png)

## 🕵️ The Mossad Agents Return

![My Cutest Mossad Agent](assets/images/kawaii-mossad-agent.png)

I thought I was done with the Mossad agents after `0.2.0`. I was not.

They came back. Same 3AM knock. Same whiteboard. But this time _they brought slides._ Twelve of them. With bullet points. And speaker notes. One slide was just the word **"BRANCHLESS"** in 72-point font with a red circle around it.

Their new requirements:

1. **[`memchr`](https://github.com/BurntSushi/memchr) for all hot-path string scanning.** _"You are doing `.contains()` in a loop like an animal",_ they said. I was.
2. **[`SmallVec`](https://github.com/servo/rust-smallvec) for version component storage.** Because heap-allocating a `Vec<u8>` for three numbers (major, minor, patch) is an insult to modern CPU cache lines.
3. **Branchless arithmetic for the bump itself.** No `if major { ... } else if minor { ... }` nonsense. One lookup table. One store. Done.
4. **Watch mode.** They wanted to know the moment a file changed. I asked why. They said that was classified.
5. **Auto-detect language manifests.** "You scan `Cargo.toml` like it's the only file in the world", they observed, correctly. "What about `pyproject.toml`? `pom.xml`? `go.mod`? Are Go developers not also suffering?"

They are. They are very much also suffering.

I implemented every point. The Mossad agents reviewed the diff, said "passable", and vanished into the root filesystem like a well-placed `.gitignore` entry.

![Ferris in black suit](assets/images/mossad-smallvec.png)

## ⚡ The 1,000,000× Number

Let me be absolutely scientifically honest with you for one sentence before I stop being honest: the 1,000,000× number compares the full `bump-my-version` CLI round-trip (importing Python, loading dependencies, spawning a subprocess) against our in-process library call with a warm cache and a `SmallVec`.

One is a sports car. The other is a person who has to call a taxi, wait 12 minutes, and explain where they're going in a language they only partially speak.

Now let's look at the real numbers.

### 🔥 New Benchmarks: `0.2.0` vs `0.2.1` vs Python

| Operation                        | `bump2version` `0.2.0` | `bump2version` `0.2.1` | Python pyO3 FFI | `bump-my-version` CLI |
| -------------------------------- | ---------------------- | ---------------------- | --------------- | --------------------- |
| Parse + bump + serialize         | ~57 µs                 | **~0.4 µs**            | ~79 µs          | ~585 ms               |
| File search/replace (1k lines)   | ~65 µs                 | **~11 µs**             | ~1.7 µs         | ~590 ms               |
| File search/replace (100k lines) | ~4.2 ms                | **~0.9 ms**            | N/A             | ~600 ms               |
| Config file parse                | ~800 µs                | **~140 µs**            | N/A             | ~500 ms (on import)   |

Versus the Python CLI subprocess: ((500 x 1000) ÷ 0.4) = **~1,250,000×** faster for a version parse-bump-serialize. We round down to 1,000,000x for humility. The Mossad agents said rounding up was acceptable. We preferred honesty.

### What got fast and why:

- **`memchr`** replaced every inner-loop string search. `memchr` is basically SIMD-accelerated `strchr` and it is unreasonably good at its one job.
- **`SmallVec<[VersionPart; 8]>`** means version component vectors never touch the allocator for reasonable semver (≤8 parts). Stack-allocated. L1 cache. Zero malloc.
- **Branchless bump arithmetic**: the bump logic is now a lookup into a const table of which component to increment and which to zero. The CPU's branch predictor doesn't even have to sweat.
- **`Arc<Regex>` cache** from `0.2.0` was kept. The regex is compiled exactly once and shared. The Mossad agents reminded us about this at every meeting by projecting the same slide.

![Ferris the crab flexin'](assets/images/soviet-ferris-smallvec.png)

## 📡 Watch Mode

> `bump --watch --bump patch`

It sounds simple. It was the opposite of simple.

"Just watch a file and re-bump when it changes", I told myself, in the tone of a man who has never used [`inotify`](https://man7.org/linux/man-pages/man7/inotify.7.html) before.

Bro, I used `inotify`. Or rather, [`notify-rs`](https://github.com/notify-rs/notify) used `inotify` for me, which is almost the same thing except someone smarter than me had already suffered through the Linux kernel filesystem event API so I wouldn't have to. God bless crate authors.

Watch mode works like this:

1. You provide a `--watch` flag.
2. `bump` registers a recursive file watcher on all files listed in your `.bumpversion.toml`.
3. Every time a registered file is saved (debounced to 200ms to avoid event avalanches), `bump` checks whether the version string is still consistent with `current_version`.
4. If it detects drift, it re-applies the bump.

This is either extremely useful for local development workflows or a deeply irresponsible footgun. We ship tools for adults. The lock safety is your problem.

```sh
# Watches VERSION, Cargo.toml, and README.md for changes, auto-bumps on save
bump --watch --bump patch --config-file .bumpversion.toml
```

```sh
# Oh no did I just? yes, yes you did. It already ran.
[watch] Detected change: Cargo.toml
[watch] Bumping patch: 0.2.1 → 0.2.2
```

![surprised pikachu ](assets/images/pikachu-sup.jpeg)

## 🌐 Multi-Language Auto-Detection: `--detect` Is Doing God's Work

Previous `bump2version` was Rust-centric. You gave it files, it bumped files. Very obedient lil tool. Very rigid. Like a Rust compiler, actually.

The new `--detect` flag changes all that:

```sh
bump --bump minor --detect
```

This single command:

1. **Walks the entire directory tree** (skipping `target/`, `node_modules/`, `.git/`, etc.)
2. **Finds every recognized manifest**: `Cargo.toml`, `pyproject.toml`, `setup.cfg`, `package.json`, `go.mod`, `pom.xml`, `build.gradle`, `Gemfile`
3. **Checks each one** for the current version string
4. **Rewrites every match** in one coordinated pass

Six languages. One command.

```sh
cd my-monorepo
bump --current-version 0.1.0 --bump patch --detect --dry-run

# [detect][dry-run] Would update: Cargo.toml
# [detect][dry-run] Would update: pyproject.toml
# [detect][dry-run] Would update: package.json
# [detect][dry-run] Would update: pom.xml
# [detect][dry-run] Would update: go.mod
# [detect][dry-run] Would commit 5 file(s): 0.1.0 → 0.1.1
```

Full supported matrix:

| Language                | Manifest Files                                |
| ----------------------- | --------------------------------------------- |
| 🦀 Rust                 | `Cargo.toml`                                  |
| 🐍 Python               | `pyproject.toml`, `setup.cfg`, `setup.py`     |
| 🟨 JavaScript / Node.js | `package.json`                                |
| 🐹 Go                   | `go.mod`                                      |
| ☕ Java                 | `pom.xml`, `build.gradle`, `build.gradle.kts` |
| 💎 Ruby                 | `Gemfile`                                     |

The Go developers, in particular, messaged to say thank you. We thanked them for using Go despite everything.

![bump2version is expanding my smol brain](assets/images/expanding-brain-detect.jpeg)

## 🌍 Real Language Examples

Every language now has a working example under [`examples/`](https://github.com/wiseaidev/bump2version/tree/main/examples), each with its own pre-baked `.bumpversion.toml`.

```sh
# Python project
bump --config-file examples/python/.bumpversion.toml --bump patch --dry-run
# [dry-run] Would commit 2 file(s): 0.1.0 → 0.1.1

# Node.js project
bump --config-file examples/nodejs/.bumpversion.toml --bump minor --dry-run
# [dry-run] Would commit 1 file(s): 0.1.0 → 0.2.0

# Java Maven project
bump --config-file examples/java/.bumpversion.toml --new-version 2.0.0 --dry-run
# [dry-run] Would commit 1 file(s): 0.1.0 → 2.0.0

# Ruby gemspec + Gemfile
bump --config-file examples/ruby/.bumpversion.toml --bump major --dry-run
# [dry-run] Would commit 2 file(s): 0.1.0 → 1.0.0

# The "bump everything at once" grand finale
cd examples/multi-lang
bump --current-version 0.1.0 --bump minor --detect --dry-run
# [detect][dry-run] Would update: Cargo.toml, pyproject.toml, package.json, pom.xml, Gemfile, VERSION
```

6 ecosystems. 1 tool. Still forbids `unsafe`. We have standards.

## 🕸️ The Yew WASM App

You ever look at a perfectly good CLI tool and think: _"This would be better if I could bump versions from a browser"?_

No? Me neither. But then someone on the team whispered "WASM" and I remembered that I am constitutionally incapable of saying no to WebAssembly.

So now there is a Yew-based web application at [`examples/yew-app/`](https://github.com/wiseaidev/bump2version/tree/main/examples/yew-app). It:

- **Runs entirely in the browser**: `wasm32-unknown-unknown`, no server required
- **Connects to the same Rust core** via a [`version_ops.rs`](https://github.com/wiseaidev/bump2version/blob/main/examples/yew-app/src/version_ops.rs) bridge module

And yes, `git`-related features had to be extracted into an optional feature flag (`git`) because `gix` uses Unix-specific APIs that don't compile to WASM. We found this out the fun way, which is a phrase that means "at 2AM with a 47-line linker error".

## 🤖 I Abused ChatGPT, Claude and Gemini. Again. More.

You may recall from my previous post that I abused Claude during `0.2.0` development. My lawyer said not to bring it up again.

So I won't bring up the fact that during `0.2.1` development, I made ChatGPT, Claude and Gemini:

- Rewrite the `detect.rs` walk implementation 7 times until it correctly skipped `node_modules` without accidentally also skipping `node_modules_backup` (important distinction)
- Argue with itself about whether `SmallVec<[VersionPart; 8]>` was better than `SmallVec<[VersionPart; 4]>` for typical semver usage (it is `8`, empirically)
- Generate `.bumpversion.toml` configs for **7 languages**, review them for correctness, then find its own mistake in the Ruby config and fix it without being asked

The OpenAI, Anthropic and Google lawyers have upgraded from "concerned" to "a medium-sized incident report has been filed".

![Prompting MS Claudet".](assets/images/prompting-ms-claudet.gif)

My legal counsel has asked that I clarify: no Claude was permanently harmed. Tokens were consumed. Electricity was used. The `--detect` flag works correctly.

## 🚀 3 Binaries, One Package, Zero Confusing Changelog Entries

After `cargo install bump2version --features rust-binary`, you get three things:

| Binary         | Use case                                                                                                                                  |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `bump`         | The modern, primary CLI                                                                                                                   |
| `cargo-bump`   | Cargo subcommand (`cargo bump --bump patch`)                                                                                              |
| `bump2version` | ✅ **Backward-compatible alias** for old CI configs, tutorials, and the 47 blog posts that told people to run `bump2version --bump patch` |

The `bump2version` binary is not deprecated. It is not going anywhere. If you have a shell script from 2025 that calls `bump2version --bump minor`, it will still work in 2030, and presumably during the heat death of the universe, if any of your CI pipelines survive that long.

This was a deliberate choice. Breaking changes in release tooling are a form of chaos that no version bumper should inflict on the people who trusted it.

```sh
# All of these do the exact same thing:
bump --bump patch
cargo bump --bump patch
bump2version --bump patch   # kept for backward compatibility
```

![Same Same](assets/images/same-picture-binaries.jpeg)

## 📋 The `.bumpversion.toml` Glow-Up

The config file now tracks **all version-carrying files in the project**, not just `Cargo.toml`:

```toml
[bumpversion]
current_version = 0.2.1
commit = false
tag = false

[bumpversion:file:Cargo.toml]
search = version = "{current_version}"
replace = version = "{new_version}"

[bumpversion:file:package.json]
search = "version": "{current_version}"
replace = "version": "{new_version}"

[bumpversion:file:README.md]
search = "{current_version}"
replace = "{new_version}"

[bumpversion:file:RUST.md]
search = "{current_version}"
replace = "{new_version}"

[bumpversion:file:WASM.md]
search = "{current_version}"
replace = "{new_version}"

[bumpversion:file:DOCKER.md]
search = `{current_version}`
replace = `{new_version}`

[bumpversion:file:PACKAGING.md]
search = {current_version}
replace = {new_version}

[bumpversion:file:rpm/bump2version.spec]
search = Version: {current_version}
replace = Version: {new_version}
```

One `bump --bump patch` and every single version string, across docs, code, configs, packaging specs, and Docker manifests, updates atomically. The Mossad agents reviewed this config and said "acceptable" which, from them, is essentially a standing ovation.

## 🦀 The Borrow Checker Tried to Ruin Watch Mode

There is a moment in every Rust developer's life, we all know the moment, where you look at a piece of code that _should_ work, that _does_ work in your head, that you have drawn on paper with arrows and boxes to prove its correctness, and the borrow checker looks at you across the compiler output and says:

> `error[E0505]: cannot move out of 'watcher' because it is borrowed`

And then below that:

> `note: the borrow later used here`

And then below _that_, a footnote that reads `note: move occurs because...` followed by a chain of reasoning so long it wraps around to the next terminal page.

The Yew WASM integration did this to me for different reasons: `gix` depends on Unix syscalls (`openat`, `statx`, platform symlink handling) that simply do not exist in `wasm32-unknown-unknown`. The linker error was 47 lines long and named three crates I had never heard of.

The fix: gating all git-related functionality behind a `git` feature flag that is disabled by default for WASM builds. Clean. Simple. The kind of solution that is obvious in retrospect and invisible before you spend four hours in the linker output.

```toml
[features]
default = ["std", "git"]
git      = ["gix", "std"]
cli      = ["clap", "git"]
rust-binary = ["cli", "git", "detect", "watch"]  # ← full featured binary
watch    = ["notify", "cli"]
detect   = ["walkdir", "cli"]
```

The WASM app uses none of the `git` features. The CLI binary uses all of them. The feature graph is clean enough that my Mossad advisors called it "elegant" before immediately asking about the benchmark numbers.

![Me excising gix.](assets/images/excising-gix.gif)

## 📦 Getting The Full Picture

```sh
# The complete CLI, with everything
cargo install bump2version --features rust-binary
```

This gets you `bump`, `cargo-bump`, and `bump2version`. All 3. No choices required. Just install and bump.

### Quick Examples

```sh
# Standard semver
bump --bump patch           # 0.2.1 → 0.2.2
bump --bump minor           # 0.2.1 → 0.3.0
bump --bump major           # 0.2.1 → 1.0.0

# Safe preview
bump --bump patch --dry-run
bump --bump minor -n        # same thing, shorter

# Git integration
bump --bump patch --commit --tag

# Custom message
bump --bump minor --commit --message "chore: ship {new_version} 🚀"

# Auto-detect all manifests
bump --bump patch --detect

# Watch mode (exits when you ctrl + c)
bump --watch --bump patch
```

### As a library (no `std` required for the core)

```toml
[dependencies]
bump2version = { version = "0.2.1", default-features = false }
```

```rust
use bump2version::{config::BumpConfig, version::{parse_version, bump_version, serialize_version}};

let cfg = BumpConfig::default();
let v   = parse_version("1.2.3", &cfg).unwrap();
let v2  = bump_version(&v, "minor", &cfg).unwrap();
assert_eq!(serialize_version(&v2, &cfg), "1.3.0");
```

### Python

```sh
pip install bump-rs
```

```python
from bump_rs import bump_version
print(bump_version("1.2.3", "major"))  # "2.0.0" in ~57µs
```

### Node.js

```sh
npm install bump2version
```

```javascript
const { bumpVersion } = require("bump2version");
console.log(bumpVersion("1.2.3", "patch")); // "1.2.4"
```

## 🚀 What Comes After 1,000,000×

Look, at some point the law of diminishing returns kicks in. We cannot make version bumping faster than the speed of light. (We checked. We asked the Mossad agents. They checked their slides. The answer was no.)

But there are things we _can_ still do:

- **Pre-release cycling**: proper `alpha.1 → alpha.2 → beta.1 → rc.1 → stable` lifecycle, first-class
- **Workspace-aware bumping**: atomic multi-crate Cargo workspace updates, all in one commit
- **More `detect` targets**: `.NET` (`*.csproj`), PHP (`composer.json`), Swift (`Package.swift`), Elixir (`mix.exs`)
- **Browser version history**: the Yew app keeps a local bump history so you can see what you broke and when

If you want any of these sooner: [open an issue](https://github.com/wiseaidev/bump2version/issues). Or just star the repo and the moral pressure will accelerate delivery. It works on me. I've tested this empirically.

## 💬 Closing Thoughts

`bump2version 0.2.1` started as a performance exercise and became an ecosystem. What was a single binary is now:

- A **Rust library** with `no_std` support for the core
- A **Python package** (`bump-rs`) for Pythonistas who want the speed without the syntax
- A **Node.js native add-on** for JavaScript developers who want to feel like they're using Rust
- A **CLI** with three binary names: `bump`, `cargo-bump`, `bump2version`
- A **WASM browser app** built in Yew, because someone had to
- **7 language examples** covering Rust, Python, Node.js, Go, Java, Ruby, and poly-repo workflows
- **20 integration tests** that test the actual binary, not a mock
- An **auto-updatable `.bumpversion.toml`** that tracks all version strings across the entire project

> `cargo install bump2version --features rust-binary` → bump → ship → repeat → be 1,000,000× faster than Python → sleep → repeat 🦀

Star [the repo](https://github.com/wiseaidev/bump2version). Try the [Python bindings](https://pypi.org/project/bump-rs). Use the [Node.js package](https://www.npmjs.com/package/bump2version). Read the [Rust docs](https://docs.rs/bump2version). Poke the [WASM demo](https://github.com/wiseaidev/bump2version/tree/main/examples/yew-app). Run the [examples](https://github.com/wiseaidev/bump2version/tree/main/examples).

This has been a public service announcement from a developer who, in the course of a single engineering session, created 20 tests, 7 example projects, a WASM frontend, a Soviet-themed benchmark suite, and consumed an amount of Claude tokens that my accountant has asked me not to disclose.

The legal proceedings with Top Tech companies remain ongoing. My lawyer has read this post. He has asked me to note, for the record, that "abusing Claude" is a colloquial and affectionate term and not a legally actionable description of token consumption.

I have not taken that advice either.

Till next time: _Keep bumpin', keep rustin', keep benchmarkin'_ 🦀⬆️

P.S. The Mossad agents have approved this post subject to the removal of the classified section about the branchless arithmetic lookup table. We kept it in. They know. They've said nothing. This is ominous.
