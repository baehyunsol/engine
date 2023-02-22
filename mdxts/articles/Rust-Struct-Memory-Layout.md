---
date: [2023, 2, 19]
tags: [rust]
preview: Rust is a low-level language. That means you can (and have to) manipulate the data at the byte level when using the language.
---

# Rust struct memory layout

Rust is a low-level language. That means you can (and have to) manipulate the data at the byte level when using the language. But, unlike C/C++, Rust programmers don't explicitly call constructors and destructors. It's difficult to know when it's destructed. How about `struct`'s padding rules? Are C/C++ padding rules also applied here? Let's see.

```rust
#[derive(Default)]
struct Foo {
    a: u64,
    b: u8,
    c: u8
}

#[derive(Default)]
struct Bar {
    a: u8,
    b: u64,
    c: u8
}

#[derive(Default)]
struct Empty {}
```

I defined 3 structs. `Foo` and `Bar` both have 3 fields, and the sum of their sizes are the same. But their order differ. An ordinary 64bits C compiler would put 7 bytes padding between `Bar.a` and `Bar.b`. A C compiler wouldn't put any padding in `Foo`. How about Rust? Let's look at their size.

```rust
fn main() {
    println!(
        "{} {} {}\n",
        std::mem::size_of::<Foo>(),
        std::mem::size_of::<Bar>(),
        std::mem::size_of::<Empty>(),
    );
}
```

The result is `16 16 0`. In C, the equivalent code prints `16 24 0`. It means Rust does some kind of optimization that C does not. Did the compiler change the order of the fields? Let's see.

```rust
let foo = Foo::default();
let bar = Bar::default();

println!(
    "foo.a: {:x}, foo.b: {:x}, foo.c: {:x}\n",
    &foo.a as *const u64 as usize,
    &foo.b as *const u8 as usize,
    &foo.c as *const u8 as usize,
);

println!(
    "bar.a: {:x}, bar.b: {:x}, bar.c: {:x}\n",
    &bar.a as *const u8 as usize,
    &bar.b as *const u64 as usize,
    &bar.c as *const u8 as usize,
);
```

Below is the result.

```
foo.a: e1a0d4f500, foo.b: e1a0d4f508, foo.c: e1a0d4f509

bar.a: e1a0d4f518, bar.b: e1a0d4f510, bar.c: e1a0d4f519
```

The compiler reordered the fields! The [language reference](https://doc.rust-lang.org/stable/reference/types/struct.html) says `struct`'s memory layout is undefined to allow such optimizations. But we can fix it with `repr`. Let's define the same struct with `repr`s.

```rust
#[repr(C)]
#[derive(Default)]
struct Foo {
    a: u64,
    b: u8,
    c: u8
}

#[repr(C)]
#[derive(Default)]
struct Bar {
    a: u8,
    b: u64,
    c: u8
}
```

With `repr(C)`, it follows the C's rules. Now `Bar` occupies 24 bytes, just like in C. Also, the fields are not reordered.

## Options

How about `Option`s? How do they look like? I guess it adds an extra field inidicating whether it's `None` or not.

```rust
println!(
    "{} {} {} {} {} {} {} {}\n",
    std::mem::size_of::<Foo>(),
    std::mem::size_of::<Option<Foo>>(),
    std::mem::size_of::<Bar>(),
    std::mem::size_of::<Option<Bar>>(),
    std::mem::size_of::<u8>(),
    std::mem::size_of::<Option<u8>>(),
    std::mem::size_of::<u16>(),
    std::mem::size_of::<Option<u16>>(),
);
```

The result is `16 24 16 24 1 2 2 4`. Well, it's interesting. `Option<Foo>` adds 8 more bytes, while `Option<u8>` adds only one byte. Is that due to the padding rules? Then, why is `Option<u16>` adding two more bytes? I guess it's related to complicated padding rules, but I can't figure out why.

While reading the std document, I found something more interesting. See below.

```rust
println!(
    "{} {} {} {}",
    std::mem::size_of::<Box<i32>>(),
    std::mem::size_of::<Option<Box<i32>>>(),
    std::mem::size_of::<&i32>(),
    std::mem::size_of::<Option<&i32>>(),
);
```

It's `8 8 8 8`. Some types don't use any field to mark whether it's null or not. Rust guarantees this kind of optimization for the types below.

- Box<U>
- &U
- &mut U
- fn, extern "C" fn
- num::NonZero*
- ptr::NonNull<U>
- #[repr(transparent)] struct around one of the types in this list.

I guess the compiler uses nullptrs to represent `None<Box<T>>` and other `None` pointers.

---

Though I mentioned constructors/destructors of Rust at the beginning, I'll talk about them in later articles.