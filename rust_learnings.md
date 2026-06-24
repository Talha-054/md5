# Rust Learnings from MD5 Implementation

---

## 1. `std::env::args()` — Command Line Arguments

Takes input from the terminal when running the program instead of stdin.

```rust
let user_input = std::env::args()
    .nth(1)
    .expect("Expected input value: Hint 'cargo run -- input-value'");
```

`std::env::args()` returns an **iterator** of all arguments passed to the program. The program name itself is at index 0, so your first real argument is at index 1.

`.nth(1)` returns `Option<String>` — not a `String` directly. Because what if the user didn't pass anything? Rust forces you to handle that case.

Usage: `cargo run -- Talha` — the `--` separates cargo's own flags from your program's arguments.

---

## 2. `Option<T>` and `.expect()`

`Option<T>` is Rust's way of saying "this value might or might not exist". It has two variants:
- `Some(value)` — value exists
- `None` — no value

```rust
.nth(1)           // returns Option<String>
.expect("msg")    // unwraps Some(value), panics with "msg" if None
```

Unlike `.unwrap()` which panics with a generic message, `.expect("msg")` lets you provide a helpful error message. Both extract the inner value from `Some`.

In JS you'd use `null` or `undefined`. Rust makes the possibility of absence explicit in the type system — you can't accidentally use a `None` as if it were a value.

---

## 3. `mod` — Module System

Tells Rust that another file exists and should be compiled as part of your program.

```rust
mod md5;  // in main.rs — loads src/md5.rs
```

Without `mod md5;`, the file `md5.rs` is completely invisible to the compiler. It won't compile it, won't type-check it, autocomplete won't work in it.

Each file in Rust is a **module** — its own namespace. Code in `main.rs` and code in `md5.rs` are separate scopes. To call something from `md5.rs` in `main.rs`:

```rust
md5::Md5::new()       // module::Type::function
md5::Md5::pad_input() // module::Type::function
```

---

## 4. `pub` — Visibility

Everything in Rust is **private by default**. Private means only code in the same file/module can use it.

```rust
pub struct Md5 { }        // public — other modules can name this type
pub fn new() -> Md5 { }   // public — other modules can call this
fn helper() { }            // private — only usable inside md5.rs
```

When `main.rs` tries to use something from `md5.rs`, Rust checks if it's `pub`. If not → compile error.

Fields inside a struct are also private by default:
```rust
pub struct Md5 {
    a: u32,   // private field — main.rs can't access hasher.a directly
    pub b: u32 // public field — main.rs can access hasher.b
}
```

This forces you to think about your public API. Only expose what the outside world needs.

---

## 5. `struct` — Custom Data Types

Groups related data together under one name. Rust's equivalent of a JS object or class (data part only).

```rust
pub struct Md5 {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}
```

Each field has a name and a type. Fields are private by default.

To create an instance (called **struct literal**):
```rust
Md5 {
    a: 0x67452301,
    b: 0xefcdab89,
    c: 0x98badcfe,
    d: 0x10325476,
}
```

You must provide every field. Order doesn't matter.

---

## 6. `impl` — Adding Behavior to a Struct

Separates data (struct) from behavior (methods). In JS these are combined in a class. In Rust they're defined separately.

```rust
struct Md5 { ... }      // data definition

impl Md5 {              // behavior definition
    fn new() -> Md5 { ... }
    fn compress(&mut self, block: &[u8; 64]) { ... }
}
```

You can have multiple `impl` blocks for the same type — Rust merges them.

---

## 7. Associated Functions vs Instance Methods

Two flavors of functions inside `impl`:

**Associated functions** — no `self`, called with `::`:
```rust
fn new() -> Md5 { ... }        // definition
md5::Md5::new()                // called with ::, no instance needed
```
Used for constructors and utilities that don't need an existing instance. `new()` is the convention for constructors but has no special status — you could name it anything.

**Instance methods** — have `self`, called with `.`:
```rust
fn compress(&mut self, block: &[u8; 64]) { ... }  // definition
hasher.compress(block)                              // called with . on instance
```

Three variants of `self`:
- `&self` — borrow the instance, read-only
- `&mut self` — borrow the instance, can modify it
- `self` — take ownership, instance is consumed (rare)

---

## 8. `Self` vs `self`

Inside an `impl` block:
- `Self` (capital S) = the **type** = `Md5`
- `self` (lowercase) = a specific **instance** of `Md5`

```rust
impl Md5 {
    fn new() -> Self { ... }           // Self = Md5
    fn do_thing(&self) {
        let copy = Self::new();        // calling associated fn via type
        println!("{}", self.a);        // accessing instance field
    }
}
```

Same relationship as a class vs an object in JS.

---

## 9. `&[u8]` — Slices (Fat Pointers)

A slice is a **view into existing data** — not an owned copy, not heap-allocated. Just a pointer + length.

```rust
fn pad_input(user_input: &[u8]) -> Vec<u8>
```

**Regular pointer** (thin): just an address — 8 bytes on 64-bit.
**Slice** (fat pointer): address + length — 16 bytes (`2 × usize`).

```
&u8    = [ptr]           ← 8 bytes  — points to one u8
&[u8]  = [ptr][len]      ← 16 bytes — points to a sequence of u8s
```

The length is baked into the reference itself. No null terminator needed (unlike C strings). Length is always O(1).

**Why `&[u8]` over `&Vec<u8>`:**
- `&Vec<u8>` only accepts a Vec
- `&[u8]` accepts Vec, fixed arrays, partial slices — anything contiguous
- Rust automatically coerces `&Vec<u8>` → `&[u8]` (deref coercion)

```rust
let v: Vec<u8> = vec![1, 2, 3];
let arr: [u8; 3] = [1, 2, 3];
pad_input(&v);      // works
pad_input(&arr);    // also works
pad_input(&v[1..]); // partial slice — also works
```

**Memory layout:**
```
Vec<u8>:  [ptr][len][cap]   ← 3 words, owns heap memory
&[u8]:    [ptr][len]        ← 2 words, borrows, zero allocation
```

`.to_vec()` — converts a `&[u8]` slice into an owned `Vec<u8>` (copies the data):
```rust
let owned = some_slice.to_vec();
```

---

## 10. `match` — Pattern Matching

Rust's powerful switch statement. Must be exhaustive — every possible value must be handled.

```rust
let k = match i / 16 {
    0 => i,
    1 => (5 * i + 1) % 16,
    2 => (3 * i + 5) % 16,
    3 => (7 * i) % 16,
    _ => unreachable!(),   // _ = catch-all, like default: in JS switch
};
```

- Each arm: `pattern => expression`
- `_` catches everything not matched above
- `match` is an **expression** — it returns a value, assigned to `k`
- All arms must return the same type

`unreachable!()` — a macro that panics with "internal error: entered unreachable code". Tells Rust "this case mathematically cannot happen" — satisfies exhaustiveness without a real handler.

---

## 11. Function Pointers

Functions in Rust have types. The type of a function taking 3 `u32`s and returning `u32`:

```rust
fn(u32, u32, u32) -> u32
```

Store a function in a variable:
```rust
let my_fn: fn(u32, u32, u32) -> u32 = Md5::f;  // no () = pointer, not a call
my_fn(1, 2, 3)  // call it later
```

Store multiple function pointers in an array:
```rust
let fns: [fn(u32, u32, u32) -> u32; 4] = [Self::f, Self::g, Self::h, Self::i];
```

Index into the array to pick and call:
```rust
fns[i / 16](self.b, self.c, self.d)
```

This replaces a match statement — instead of 4 arms calling 4 different functions, one line picks and calls the right one by index.

For MD5's round groups:
```
i/16 = 0 → fns[0] = F  (rounds 0–15)
i/16 = 1 → fns[1] = G  (rounds 16–31)
i/16 = 2 → fns[2] = H  (rounds 32–47)
i/16 = 3 → fns[3] = I  (rounds 48–63)
```

Integer division truncates: `17 / 16 = 1`, `5 / 16 = 0`. Exactly what's needed.

---

## 12. Closures `|params| expression`

Anonymous functions — like arrow functions in JS.

JS:   `(i) => i * 2`
Rust: `|i| i * 2`

With a block body:
```rust
|i| {
    let x = i * 2;
    x + 1   // last expression is the return value, no semicolon
}
```

Parameters go between `|` pipes. Used heavily with iterators.

---

## 13. `std::array::from_fn` — Building Arrays with Closures

Builds a fixed-size array by calling a closure once per index:

```rust
let t_constants: [u32; 64] = std::array::from_fn(|i| Self::compute_t(i));
```

Rust calls the closure with `i = 0, 1, 2, ... 63`. The return value of each call becomes the element at that index. The size (64) is inferred from the type annotation.

Replaces the manual loop:
```rust
// old way
let mut arr = [0u32; 64];
for i in 0..64 { arr[i] = compute_t(i); }

// from_fn way — one line, no mutation needed
let arr: [u32; 64] = std::array::from_fn(|i| compute_t(i));
```

---

## 14. Tuple Destructuring

Unpack multiple values from a tuple in one line:

```rust
let (aa, bb, cc, dd) = (self.a, self.b, self.c, self.d);
```

Right side creates a tuple of 4 values. Left side destructures it into 4 separate bindings. One line instead of four.

Tuples in Rust: `(value1, value2, value3)` — fixed size, can mix types.

Access by index: `tuple.0`, `tuple.1`, `tuple.2`.

---

## 15. `f64::sin()` and Related Math Methods

```rust
f64::sin((i + 1) as f64)   // sine — takes radians, returns -1.0 to 1.0
.abs()                      // absolute value — makes negative numbers positive
.floor()                    // rounds DOWN to nearest integer (keeps as f64)
```

MD5 T constants: `floor(abs(sin(i+1)) * 2^32)`

Why multiply by `2^32` (= 4294967296)?  
`sin()` returns a fraction between 0 and 1. Multiplying by `2^32` **scales** that fraction into the full range of a `u32` (0 to 4294967296). Then `floor()` chops the decimal. Then `as u32` stores it.

It is NOT "extracting first 32 bits" — it is purely **scaling a fraction into the u32 range**. The SHA-256 whitepaper's wording of "first 32 bits of the fractional part" describes the same operation in binary terms but causes confusion. MD5's RFC is more honest: "multiply by 2^32 and take integer part."

---

## 16. `usize` — Platform-Dependent Integer

`usize` is the pointer-sized integer. On 64-bit machines it's 64 bits. On 32-bit machines it's 32 bits.

`.len()` always returns `usize`. Array/Vec indices are always `usize`.

Key rule: `usize * usize = usize`. The type of the result depends on the types going in, not the size of the result coming out. To get a `u64` from a `usize`:

```rust
let len_u64 = vec.len() as u64;   // cast first
let bits = len_u64 * 8;           // now multiplication is in u64-land
```

---

## 17. `to_le_bytes()` vs `to_be_bytes()` and `from_le_bytes()` vs `from_be_bytes()`

Direction:
- `from_*` — bytes **into** a number: `[u8; 8] → u64`
- `to_*` — number **into** bytes: `u64 → [u8; 8]`

Endianness:
- `be` (big-endian) — most significant byte first: `[0, 0, 0, 0, 0, 0, 0, 40]` for 40
- `le` (little-endian) — least significant byte first: `[40, 0, 0, 0, 0, 0, 0, 0]` for 40

MD5 is **little-endian** throughout — padding length, word parsing, output.  
SHA-256 is **big-endian** throughout.

```rust
// number → bytes (for appending to Vec)
some_u64.to_le_bytes()    // returns [u8; 8]

// bytes → number (for parsing message block)
u32::from_le_bytes(chunk.try_into().unwrap())   // [u8; 4] → u32
```

The `[u8; 8]` return type is a **fixed array** not a Vec — size is always exactly 8, known at compile time, guaranteed by the type. That's why `from_le_bytes` takes `[u8; 8]` and not `&[u8]` — the compiler enforces correct size at compile time, no runtime check needed.

---

## 18. `const` at Module Level

A compile-time constant baked into the binary. Available everywhere in the file. Evaluated at compile time — zero runtime cost.

```rust
const SHIFT_CONSTANTS: [u32; 64] = [
    7, 12, 17, 22, ...
];
```

Rules:
- Must have explicit type annotation
- ALL_CAPS naming convention
- Value must be computable at compile time (no `f64::sin()` — floating point not allowed in const context)
- Lives for the entire program

Different from `let` which is runtime. `const` is purely compile-time.

---

## 19. Type Casting Full Mental Model (`as`)

`as` reinterprets a value as a different type. Always compiles — no safety checks.

**Upcasting (safe, no data loss):**
```rust
let a: u8 = 100;
let b: u32 = a as u32;   // 100 → 100, box is bigger, zeros pad the front
let c: u64 = a as u64;   // 100 → 100
let d: f64 = a as f64;   // 100 → 100.0
```

**Downcasting (truncates bits):**
```rust
let a: u32 = 300;
let b: u8  = a as u8;   // 300 → 44 — bits get cut, NO WARNING from Rust
```
300 in binary is `100101100`. u8 can only hold 8 bits → `00101100` = 44.

**Float → integer (truncates decimal, no rounding):**
```rust
let a: f64 = 9.99;
let b: u32 = a as u32;   // 9 — decimal chopped, not rounded
```

**Signed ↔ Unsigned (reinterprets bits):**
```rust
let a: u8 = 255;
let b: i8 = a as i8;   // -1 — same bits, read as signed
```

The **bits don't change** — only how Rust interprets them changes.

**`usize` casting:**
```rust
vec.len() as u64   // always safe — usize never bigger than u64 on any platform
vec.len() as u8    // almost always truncates — dangerous
```

Safe alternative using `From` trait (only works for lossless conversions):
```rust
u32::from(some_u8)   // compiles — u8 always fits in u32
// u8::from(some_u32) — COMPILER ERROR — would lose data
```

---

## 20. Bitwise NOT `!` on integers

In SHA-256 you used `!` on booleans. In MD5, `!` on integers flips every bit:

```rust
!0b00001111u8 = 0b11110000   // every 0 becomes 1, every 1 becomes 0
!b   // where b: u32 — flips all 32 bits
```

Used in auxiliary functions:
```rust
(b & c) | (!b & d)   // F function — NOT b, then AND with d
```

Different from `!` on `bool` which just flips true/false. On integers it's a full bitwise complement.

---

## 21. Slice Index Assignment with `.copy_from_slice()`

You cannot assign a fixed array directly into a slice range:

```rust
hash[0..4] = self.a.to_le_bytes();   // COMPILER ERROR
```

Rust doesn't allow direct slice assignment with `=`. Use `.copy_from_slice()` instead:

```rust
hash[0..4].copy_from_slice(&self.a.to_le_bytes());
```

`hash[0..4]` gives a mutable slice of 4 elements. `.copy_from_slice()` copies exactly that many bytes from the source into it. The source must be the same length — compiler won't catch this, it panics at runtime if lengths differ.

---

## 22. Stack vs Heap — When to Use Which

**Stack** — size must be known at **compile time**:
```rust
let arr: [u32; 16] = ...   // size 16, fixed, known at compile time → stack
```
- Zero allocation cost
- Automatically freed when it goes out of scope
- Fast access

**Heap** — size can be determined at **runtime**:
```rust
let v: Vec<u8> = Vec::new();   // size unknown at compile time → heap
```
- Allocation cost (malloc under the hood)
- Must be explicitly managed (Rust does it via ownership/drop)
- Flexible — can grow and shrink

The fundamental rule: **if the size depends on user input or runtime data, you need heap**. You cannot avoid it for `pad_input` because the padded size depends on how long the user's input is — unknown until the program runs.

Everything else in MD5 — block words `[u32; 16]`, digest `[u8; 16]`, T constants `[u32; 64]`, shift table `[u32; 64]` — is fixed size, lives on the stack, zero heap allocation.
