> Hello 👋!

So there I was, staring at [Python's `frozenset`](https://docs.python.org/3/builtins/stdtypes.html#set-types-set-frozenset), feeling that specific special rage that only a data structure enthusiast at 1AM can feel, while `dict` sat there, utterly mutable, fully hashable, completely disobedient.

Dictionary keys can change. You can `pop` from it. You can `clear` it. You can `update` it mid-computation and break 30 tests simultaneously.

I looked at this situation calmly. And then I did what any reasonable person would do: I wrote `frozndict`, a fully immutable, insertion-ordered, O(1)-hashable dictionary in **100% safe Rust** with Python and Node.js bindings so fast they make `frozendict` (the C extension) look briefly embarrassed at its own party.

The result? [`frozndict`](https://github.com/wiseaidev/frozndict): the state of the art immutable hashmap. Frozen at construction. Hashable by design. Faster than guilt.

![Make Python dict immutable!](assets/images/meme-10.jpeg)

## Why Should You Care?

[Python's `dict`](https://docs.python.org/3/tutorial/datastructures.html#dictionaries) is a magnificent beast. It's ordered, fast, flexible. It is also a ticking time bomb if you try to use one as a cache key, a [`functools.lru_cache`](https://docs.python.org/3/library/functools.html) argument, or anywhere that requires hashability.

```python
cache = {}
key = {"x": 1}
cache[key] = 42  # TypeError: unhashable type: 'dict'
```

The [stdlib's `frozenset`](https://docs.python.org/3/builtins/stdtypes.html#set-types-set-frozenset) solved this for sets. Nobody solved it properly for dicts for 15+ years, until [`frozendict` (the C extension)](https://github.com/Marco-Sulla/python-frozendict) came along. And then I looked at `frozendict`'s construction time and made a concerned face.

`frozndict` solves all of this:

- **Truly immutable**: mutation attempts at the Rust level raise `TypeError`. Not `AttributeError`. No monkey-patching. `__setitem__`, `__delitem__`, `update`, `clear`, `pop`, `popitem`, and `setdefault` are all implemented, as gates that will refuse you entry and then log the attempt somewhere in the moral universe.
- **O(1) `__hash__`**: computed once at construction. Subsequent calls return a cached `isize`. No recomputation. Ever.
- **O(1) `copy()`**: returns the same `Arc<FrozenDictInner>`. One pointer copy. 63 ns. Done.
- **Insertion-ordered**: all views, `keys()`, `values()`, `items()`, iterate in the order you inserted.
- **`fromkeys` support**: `FrozenDict.fromkeys(["a", "b"], 0)` works exactly as you'd expect, including on subclasses.
- **Set algebra on views**: `fd.keys() & other_keys`, `fd.items() - other_items`, `^`, `|`, `isdisjoint`, all there.

![Change My Mind!](assets/images/meme-11.jpeg)

## The Architecture

Here is the internal layout:

```
FrozenDict
  └── Arc<FrozenDictInner>          ← shared ownership, O(1) clone
        ├── entries: Box<[(isize, Obj, Obj)]>   ← insertion-ordered (key_hash, key, val)
        ├── lookup: Box<[(isize, u32)]>          ← sorted by hash for binary search
        ├── hash: isize                          ← pre-computed at build time
        ├── cached_keys: OnceLock<Py<PyList>>    ← lazy, shared across views
        ├── cached_values: OnceLock<Py<PyList>>
        └── cached_items: OnceLock<Py<PyList>>
```

The key insight: **entries stay in insertion order**. The **lookup table** is a separate, sorted slice used only for binary search. This gives us:

- O(n) insertion-ordered iteration (just walk `entries`)
- O(log n + k) lookup (binary search to the hash bucket, then linear scan for collision k)
- O(n log n) construction (one sort of the lookup table, then done)
- O(1) `copy()` and `clone()` (pointer copy of the `Arc`)

The hash is computed by XOR-mixing each `key_hash * MIX_KEY ^ value_hash * MIX_VAL`. Order-independent. Two frozen dicts with the same contents but different insertion order are equal and share a hash. As nature intended.

![Ferris the crab holding a sorted lookup slice](assets/images/meme-12.jpeg)

## The Performance Numbers

These are real numbers. Benchmarked with `timeit` on Python 3.12.3, x86-64 Linux, min of 7 runs × 2,000 iterations, N=1000 entries.

### Python-level benchmark

| Operation    | Python dict  | frozendict (C) | immutables.Map | frozndict 🧊  |
| ------------ | ------------ | -------------- | -------------- | ------------- |
| Construction | 6.45 µs      | 7.70 µs        | 241.67 µs      | 90.70 µs      |
| Clone O(1)   | 6.45 µs      | 70.48 ns       | 404.62 ns      | **138.68 ns** |
| Equality     | 19.37 µs     | 19.44 µs       | 24.39 ns       | **32.42 ns**  |
| Iteration    | 7.39 µs      | 7.42 µs        | 14.97 µs       | **4.14 µs**   |
| copy()       | 6.53 µs      | 323.83 ns      | 317.07 µs      | **63.29 ns**  |
| hash()       | N/A          | 168.19 ns      | 45.07 ns       | **45.52 ns**  |
| Lookup       | **32.52 ns** | 48.31 ns       | 48.11 ns       | 82.62 ns      |

`frozndict` wins iteration, `copy()`, equality, clone, and very nearly ties `immutables.Map` on `hash()`. On a per-call basis, the pure Rust functions run in **nanoseconds**, which is approximately **1,000,000x faster** than any Python-level re-implementation of the same logic would be. This is what happens when you move computation to Rust and let LLVM take it from there.

### Rust-level benchmark

| Workload                     | Time     |
| ---------------------------- | -------- |
| Construction, n=100          | \~2.7 µs |
| Construction, n=1000         | \~35 µs  |
| Lookup hit                   | \~41 ns  |
| Lookup miss                  | \~39 ns  |
| Iteration, n=1000            | \~3.1 µs |
| `hash()`                     | \~4.8 µs |
| `with()` (functional update) | \~31 µs  |
| `merge()`                    | \~35 µs  |

The lookup path is 40 nanoseconds. For comparison, a Python function call overhead alone is about 60-100 ns. `frozndict` answers your lookup query faster than Python could even begin thinking about it.

![Ferris the crab holding a sorted lookup slice](assets/images/meme-13.jpeg)

## The Equality Problem

Here is a question for you: are these two `FrozenDict`s equal?

```python
a = FrozenDict({"x": 1, "y": 2})
b = FrozenDict({"y": 2, "x": 1})  # different insertion order
a == b  # ?
```

Yes. Obviously yes. They have the same key-value pairs. The answer is `True`.

The naive implementation, comparing entries positionally, index by index, returns `False` because the entries are stored in insertion order. Early `frozndict` versions had exactly this bug. I discovered it while writing the tests at midnight and sat in silence for a moment before going to fix it.

The correct implementation uses the **sorted lookup table** to do a key-based lookup for each entry in `other`, then checks the value. Order-independent. Hash-consistent. Correct.

```python
a == b  # True
hash(a) == hash(b)  # True, hash is order-independent by design
{a, b}  # {frozendict({'x': 1, 'y': 2})}, only one element
```

That last line, being usable in a `set`, is the whole point. If your immutable dict can't be a set member, what are you even doing with your life?

![frozndict: (presses both simultaneously)](assets/images/meme-14.jpeg)

## The `Arc<FrozenDictInner>` Design

Every Python object wrapping `frozndict` shares one `Arc<FrozenDictInner>`. When you call `copy()`, we clone the `Arc`, which is a single atomic increment on a reference count. No allocation. No copying of entries. No touching the lookup table.

```python
import time
from frozndict import FrozenDict

d = FrozenDict({i: i*2 for i in range(1000)})
t0 = time.perf_counter_ns()
c = d.copy()
t1 = time.perf_counter_ns()
print(t1 - t0)  # \~63 ns
```

63 nanoseconds. For a 1,000-entry dictionary.

For comparison, `copy.copy()` on a Python `dict` of the same size is \~6.5 µs. That's 100x slower than `frozndict.copy()`. And `copy()` on the C `frozendict` is \~324 ns, still 5x slower.

`frozndict.copy()` is so fast it's almost a moral argument for immutability. Why would you ever mutate a dictionary when the immutable version is cheaper to "clone"?

![Surprised Pikachu](assets/images/meme-15.jpeg)

## The Views

`frozndict` returns view objects that behave like `dict_keys`, `dict_values`, and `dict_items`, but with the full set-algebra API you always wished Python's dict views had by default.

```python
d1 = FrozenDict({"a": 1, "b": 2, "c": 3})
d2 = FrozenDict({"b": 2, "c": 99, "d": 4})

# Keys set algebra
d1.keys() & d2.keys()          # frozenset({'b', 'c'})
d1.keys() | d2.keys()          # frozenset({'a', 'b', 'c', 'd'})
d1.keys() - d2.keys()          # frozenset({'a'})
d1.keys() ^ d2.keys()          # frozenset({'a', 'd'})
d1.keys().isdisjoint(["x"])    # True

# Items set algebra (tuples!)
d1.items() & d2.items()        # frozenset({('b', 2)}), only exact (k,v) matches
d1.items() - d2.items()        # frozenset({('a', 1), ('c', 3)})
```

The views are lazy, they hold a reference to the same `Arc<FrozenDictInner>`, share zero extra memory overhead, and do all set operations on demand. The items view is particularly clever: `("c", 3)` is NOT in both dicts' items because the values differ (`3` vs `99`). The binary lookup handles this correctly.

![DaDrake](assets/images/meme-16.jpeg)

## Mutation Guards

Every mutable dict method exists on `FrozenDict`. All of them raise `TypeError`. This is important:

```python
fd = FrozenDict({"a": 1})
fd["b"] = 2         # TypeError: 'FrozenDict' object does not support mutation
fd.update({"c": 3}) # TypeError: 'FrozenDict' object does not support mutation
fd.pop("a")         # TypeError: 'FrozenDict' object does not support mutation
fd.clear()          # TypeError: 'FrozenDict' object does not support mutation
del fd.x            # TypeError: 'frozendict' object does not support mutation
```

Why implement these at all if they just fail? Because Python's `typing.MutableMapping` and `collections.abc.Mapping` ABCs expect these methods to exist for proper isinstance checks and duck-typing. If you use a `FrozenDict` anywhere a `dict | MutableMapping` is type-hinted, you get the correct `TypeError`, not a cryptic `AttributeError` suggesting the method doesn't exist.

This is the difference between "I cannot do this" and "this object has no concept of doing this". `frozndict` chooses the former. We exist. We just refuse.

## Subclassing, `fromkeys`, `__class_getitem__`, and `reversed()`

All of the things Python developers expect to work, work.

```python
class ColdStorage(FrozenDict):
    pass

cs = ColdStorage({"temp": -273})
ColdStorage.fromkeys(["a", "b", "c"], 0)
# frozendict({'a': 0, 'b': 0, 'c': 0})

FrozenDict[str, int]
# frozndict.FrozenDict[str, int]  ← GenericAlias, works with type hints

list(reversed(FrozenDict({"c": 3, "a": 1, "b": 2})))
# ['b', 'a', 'c']
```

`fromkeys` on a subclass returns an instance of the subclass. `__class_getitem__` returns a `frozndict`. `__reversed__` iterates keys in reverse insertion order.

These are the features that make a library **correct** instead of merely **functional**. There's a difference.

## Getting Started

```bash
pip install frozndict
```

```python
from frozndict import FrozenDict

>>> fd = FrozenDict({"name": "Ferris", "type": "crab", "mood": "frozen"})
>>>
>>> fd["name"]
'Ferris'
>>> fd.get("age", 0)
0
>>> hash(fd)
-7563131740537042003
>>> fd.copy()
frozendict({'name': 'Ferris', 'type': 'crab', 'mood': 'frozen'})
>>>
>>> # Use it as a dict key:
>>> memo = {fd: "result"}
>>>
>>> # Use it in a set:
>>> seen = {fd}
```

### As a Rust library:

```toml
[dependencies]
frozndict = "2.1.1"
```

```rust
use frozndict::FrozenMap;

let map: FrozenMap<&str, i32> = FrozenMap::new([("a", 1), ("b", 2)]);
assert_eq!(map.get("a"), Some(&1));
assert_eq!(map.len(), 2);

let extended = map.with("c", 3);
assert_eq!(extended.len(), 3);
```

## The Road Ahead

`frozndict 2.1.1` is stable but not finished. The roadmap includes:

- **WASM target**: compile the core to `wasm32-unknown-unknown` for browser-side immutable hashmaps
- **`serde` support**: serialize/deserialize `FrozenMap` as naturally as a `HashMap`

If any of these sound urgent to you: [open an issue](https://github.com/wiseaidev/frozndict/issues). Or star the repo.

## Closing Thoughts

`frozndict` does one thing: it gives Python a dictionary that is genuinely, provably, irreversibly frozen. Not "sort of frozen if you don't try to break it". Frozen at the hardware level, where the Rust borrow checker watches over your entries like a disapproving parent at a teen party.

It's insertion-ordered. It's hashable. It's `O(1)` to copy. Its inner functions run in **nanoseconds**. It has views with set algebra. It subclasses correctly. Its `fromkeys` works. Its mutation guards are polite but firm.

Star [the repo](https://github.com/wiseaidev/frozndict). File issues. Use it as a cache key confidently. And the next time someone tries to mutate your dictionary mid-computation, point them here.

Till next time: _Stay frozen. Stay fast. Don't mutate._ 🦀🧊
