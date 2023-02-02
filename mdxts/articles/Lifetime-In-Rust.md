---
date: [2023, 1, 31]
tags: [rust]
preview: Lifetime is one of the most important concept in the Rust world. It guarantees that a pointer is always pointing to a valid memory.
---

# How lifetimes work in Rust

Lifetime is one of the most important concept in the Rust world. It guarantees that a pointer is always pointing to a valid memory. The safety guarantee it gives makes the language very powerful, but it's difficulty make young programmers hesitate to become Rustacean. I was one among them. I've been using this language for years (all the frameworks for this blog is written in Rust), but still not sure how lifetimes work. This blog post is a complete guide for the Rust lifetimes, for myself and others.

> Most of this post is a summary of the [official Rust book](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html). It helps me understand the concept by summarizing it. If you also want to understand the concept, you should read the book, not this article.

## What is a lifetime?

[[giant]]Lifetime is a part of the type system.[[/giant]] Every reference in Rust has a lifetime telling how long does the reference live. That means `&u32` that lives long and `&u32` that lives short are different types. If a struct has references as it's field, the struct also has a lifetime. Structs with different lifetimes are regarded as different types.

The biggest role of lifetimes is preventing dangling references. Let's look at an example below.

```rust, line_num
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}

```

In the example, `x` goes out of scope at line 7, and `r` tries to read the value of `x` at line 9. It seems obvious that the program is wrong, but how does the compiler know that? The compiler checks the *lifetime* of each value, and makes sure that the original value lives longer than it's references. It's illustrated below.

```rust, line_num
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

`'a` is the lifetime of `r`, and `'b` is of `x`. A lifetime starts when a value is initialized, and ends when it's dropped. Since `r` is referencing `x`, the compiler wants the lifetime of `x` to be longer than `r`. The compiler refuses the above code because `x` doesn't *live long enough*.

We call it *Borrow-Checker*. It checks all the *borrows* (references) live long enough, so that there's no dangling pointers.

That's it. It's how the Rust compiler utilizes lifetimes. It's a very simple concept, but it gets complicated when the code becomes longer.

## Lifetimes of function parameters

Let's define a function. It takes two references as it's input, and returns the bigger one.

```rust, line_num
fn big(m: &u32, n: &u32) -> &u32 {
    if *m < *n { n } else { m }
}
```

The code is straightforward, but the compiler doesn't allow it. Below is the error message.

```
error[E0106]: missing lifetime specifier
 --> main.rs:1:29
  |
1 | fn big(m: &u32, n: &u32) -> &u32 {
  |                             ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `m` or `n`
```

The return value of the function is either `m` or `n`, which might have different lifetimes. If the compiler doesn't know the lifetime, the borrow-checker will not work! I'll show you a counter-example.

```rust, line_num
fn main() {
    let x = input();       // --------+--'a
    let z;                 //         |
    {                      //         |
        let y = input();   // -+--'b  |
                           //  |      |
        z = big(&x, &y);   //  |      |
    }                      // -+      |
                           //         |
    println!("z: {}", z);  //         |
}                          // --------+
```

If `big` returns `&y`, it's dropped before `println!` is called, which is problematic. If `big` returns `&x`, there's no problem. But it's impossible to know whether `&x` or `&y` will be chosen at compile-time. So the compiler refueses this possibly-problematic code. How do we fix this? We first have to add lifetime annotations to the function declaration, like below.

```rust, line_num
fn big<'a, 'b>(m: &'a u32, n: &'b u32) -> &'a u32 {
    if *m < *n { n } else { m }
}
```

It's our first attempt. I gave different lifetimes to `m` and `n` because they might have different ones. But in that case, we cannot annotate the lifetime of the return value.

```
error[E0623]: lifetime mismatch
 --> main.rs:2:18
  |
1 | fn big<'a, 'b>(m: &'a u32, n: &'b u32) -> &'a u32 {
  |                               -------     -------
  |                               |
  |                               this parameter and the return type are declared with different lifetimes...
2 |     if *m < *n { n } else { m }
  |                  ^ ...but data from `n` is returned here
```

What the compiler is complaining is that, the type of the return value should be `&'a u32`, but line 2 returns `&'b u32`. How do we fix it? Let's give it another try.

```rust, line_num
fn big<'a>(m: &'a u32, n: &'a u32) -> &'a u32 {
    if *m < *n { n } else { m }
}
```

We gave the same lifetime to `m` and `n`. Though it compiles, it sounds a bit strange. In most cases, `m` and `n` would have different lifetimes, but why are we giving the same lifetimes to them? That's because we're giving long enough lifetimes, not an exact lifetime. So the lifetime annotation means ~_`m` is valid in `'a`, `n` is valid in `'a`, and the return value is valid in `'a`._~

```rust, line_num
fn main() {
    let x = input();       // --------+--'a
    let z;                 //         |
    {                      //         |
        let y = input();   // -+--'b  |
                           //  |      |
        z = big(&x, &y);   //  |      |
    }                      // -+      |
                           //         |
    println!("z: {}", z);  //         |
}                          // --------+
```

What's the lifetime of the input values of `big` in this code? It must be either `'a` or `'b`. If it're `'a`, it doesn't make sense because `y` dies before `'a` finishes. If it're `'b`, it makes sense because both are valid in `'b`. So the compiler gives `'b` to the return value of `big`. It tells that the return value is valid in `'b`, but it'd be dangerous to use the value after `'b`. That's why the compiler doesn't let us assign `big` to `z`.

### Lifetimes of structs

If a struct holds references, the struct needs a lifetime annotation.

```rust, line_num
struct RefInt<'a> {
    i: &'a u32
}

fn main() {
    let x = 3;                // ---------+--'b
    let s = RefInt { i: &x }; // -+--'c   |
                              //  |       |
    println!("{}", s.i);      // -+-------+
}
```

It compiles! The compiler has to make sure that `RefInt` never owns an uninitialized pointer. So the original value of `i` must live longer than the instance of `RefInt`. In the code, the original value `x` lives longer than `s` and the code compiles.

The lifetime annotations of structs are read in this way: ~_`i` is valid in `'a` and the instance is valid in `'a`._~ In the code, `i` is valid in `'b` and `s` is valid in `'b`. They both are valid in `'c`, so the compiler accepts our code.

## Lifetime ellision

In earlier versions of Rust, we had to specify the lifetime of every reference. That sounds very cumbersome, doesn't it? The compiler tries to infer the lifetime with a few rules. Though it only has 3 rules, more rules will be added as the language evolves. Below are the rules.

1. The compiler assigns lifetimes to each reference.
  - `fn foo(n: &u32)` -> `fn foo<'a>(n: &'a u32)`
  - `fn foo(n: &u32, m: &u32)` -> `fn foo<'a, 'b>(n: &'a u32, m: &'b u32)`
1. If there's only one lifetime parameter in the input, all the outputs have that lifetime.
  - `fn foo<'a>(n: &'a u32) -> &u32` -> `fn foo<'a>(n: &'a u32) -> &'a u32`
1. If a method has a reference to `self`, (ex: `&self`, `&mut self`), the lifetime of `self` is assigned to all the output lifetime parameters.

The second and the third ones are the most important. They allow us to omit lifetime parameters in most cases. Let's look at why those rules make sense.

When a function returns a reference, the reference has to be valid at the moment it's returned. But all the values owned by the functions are dropped when the function returns. That means the return value has to reference something that's not owned by the function. What can be such thing? Let's look at some examples.

```rust, line_num
fn func1(n: &u32) -> &u32 {
    n
}

fn func2<'a>() -> &'a u32 {
    let x = 3;
    &x
}
```

`func1` compiles but `func2` does not. `n` and the return value of `func1` have the same lifetime as you see in its body. But `func2` raises an error like below.

```
error[E0515]: cannot return reference to local variable `x`
 --> main.rs:7:5
  |
7 |     &x
  |     ^^ returns a reference to data owned by the current function

error: aborting due to previous error
```

The value `x` is dropped at line 8, so the reference to `x` cannot be used by others.

Are there execptions for the rule? Let's make a function that returns a reference that's not an input.

```rust, line_num
fn func3(_: &str) -> (&str, String) {
    let x = String::from("abc");

    (&x, x)
}
```

I wanted to return a value created in the function. If the value is not dropped at the end of the function, reference must be valid! In order to prevent from being dropped, I have to return the value (moving it's ownership). Does it work? Well... look at the error messages below.

```
error[E0515]: cannot return value referencing local variable `x`
 --> main.rs:4:5
  |
4 |     (&x, x)
  |     ^--^^^^
  |     ||
  |     |`x` is borrowed here
  |     returns a value referencing data owned by the current function

error[E0505]: cannot move out of `x` because it is borrowed
 --> main.rs:4:10
  |
1 | fn func3(_: &str) -> (&str, String) {
  |             - let's call the lifetime of this reference `'1`
...
4 |     (&x, x)
  |     -----^-
  |     ||   |
  |     ||   move out of `x` occurs here
  |     |borrow of `x` occurs here
  |     returning this value requires that `x` is borrowed for `'1`
```

The compiler tells me that what I'm trying to do is impossible. When moving `x` out of the function, the reference is invalidated. What if we change the shape of it's output?

```rust, line_num
fn func4(_: &str) -> (String, &str) {
    let x = String::from("abc");

    (x, &x)
}
```

I hope moving `x` doesn't validate `&x` in this code. Does it?

```
error[E0515]: cannot return value referencing local variable `x`
 --> main.rs:4:5
  |
4 |     (x, &x)
  |     ^^^^--^
  |     |   |
  |     |   `x` is borrowed here
  |     returns a value referencing data owned by the current function

error[E0382]: borrow of moved value: `x`
 --> main.rs:4:9
  |
2 |     let x = String::from("abc");
  |         - move occurs because `x` has type `std::string::String`, which does not implement the `Copy` trait
3 | 
4 |     (x, &x)
  |      -  ^^ value borrowed here after move
  |      |
  |      value moved here

error: aborting due to 2 previous errors
```

It doesn't even let me reference `x` because `x` is moved before referenced. Well I guess it's impossible to return such reference.

### More Examples

Let's look at more complicated example. I'll define a struct that holds a reference to itself. Then I'll try to create a dangling pointer by messing up with the struct. Let's hack!

```rust, line_num
struct RefSelf<'a> {
    vec: Vec<u32>,
    refs: Vec<&'a u32>
}
```

Well, it compiles! Now, let's make `refs` reference the values of `vec`.

```rust, line_num
impl<'a> RefSelf<'a> {

    pub fn new() -> Self {
        let vec = vec![0; 10];
        let refs = vec![&vec[0]];

        RefSelf { vec, refs }
    }

}
```

It's the most naive way I can think of. It fails, saying that it cannot move out of `vec` because it's borrowed. It's what we've seen in the error messages of `func3`. Let's try more deliberate way.

```rust, line_num
impl<'a> RefSelf<'a> {

    pub fn new() -> Self {
        let vec = vec![0; 10];
        let refs = vec![];

        RefSelf { vec, refs }
    }

    pub fn add(&'a mut self) {
        self.refs.push(&self.vec[0]);
    }

    pub fn clear(&'a mut self) {
        self.vec.clear();
    }

}
```

It compiles! My plan is to print `&self.vec[0]` after clearing `self.vec`. If it compiles, it'd print an uninitialized value (a value of a freed memory).

```rust, line_num
fn main() {
    let mut refself = RefSelf::new();
    refself.add();
    println!("{}", refself.refs[0]);
    refself.clear();
    println!("{}", refself.refs[0]);
}
```

Does it successfully destroys the borrow checker? Look at the result below.

```
error[E0502]: cannot borrow `refself.refs` as immutable because it is also borrowed as mutable
 --> main.rs:4:20
  |
3 |     refself.add();
  |     ------- mutable borrow occurs here
4 |     println!("{}", refself.refs[0]);
  |                    ^^^^^^^^^^^^
  |                    |
  |                    immutable borrow occurs here
  |                    mutable borrow later used here

error[E0499]: cannot borrow `refself` as mutable more than once at a time
 --> main.rs:5:5
  |
3 |     refself.add();
  |     ------- first mutable borrow occurs here
4 |     println!("{}", refself.refs[0]);
5 |     refself.clear();
  |     ^^^^^^^
  |     |
  |     second mutable borrow occurs here
  |     first borrow later used here

error[E0502]: cannot borrow `refself.refs` as immutable because it is also borrowed as mutable
 --> main.rs:6:20
  |
3 |     refself.add();
  |     ------- mutable borrow occurs here
...
6 |     println!("{}", refself.refs[0]);
  |                    ^^^^^^^^^^^^
  |                    |
  |                    immutable borrow occurs here
  |                    mutable borrow later used here

error: aborting due to 3 previous errors
```

What's going on? When `refself.add` is called, it asks `&'a mut self` as an input. After that, `refself` holds the mutable reference of itself. While the mutable reference exists, no one can make a reference of `refself`. It's very important rule: a mutable reference must be the only reference. The mutable reference's lifetime is `'a`, which is the lifetime of `refself`. So any reference to `refself` after line 3 is an error.

What if we change the declaration of the methods?

- `pub fn add(&'a mut self)` -> `pub fn add(&mut self)`
- `pub fn clear(&'a mut self)` -> `pub fn clear(&mut self)`

It lets the compiler choose the lifetime of `&mut self` more flexibly. Can I break the borrow checker this time? No...

```
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in function call due to conflicting requirements
  --> main.rs:24:25
   |
24 |         self.refs.push(&self.vec[0]);
   |                         ^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 23:5...
  --> main.rs:23:5
   |
23 | /     pub fn add(&mut self) {
24 | |         self.refs.push(&self.vec[0]);
25 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> main.rs:24:25
   |
24 |         self.refs.push(&self.vec[0]);
   |                         ^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'a as defined on the impl at 14:6...
  --> main.rs:14:6
   |
14 | impl<'a> RefSelf<'a> {
   |      ^^
note: ...so that reference does not outlive borrowed content
  --> main.rs:24:24
   |
24 |         self.refs.push(&self.vec[0]);
   |                        ^^^^^^^^^^^^

error: aborting due to previous error
```

We're seeing a brand-new error type this time. In this case, the compiler gives a lifetime `'b` to `&mut self` of `fn add(&mut self)` (rule 1). The second error message is telling us that `&self.vec[0]` cannot outlive `'b`, because that's the lifetime of `self`. And the forth error message is telling us that `&self.vec[0]` must be valid for `'a`, because that's the lifetime of `self`. Since the compiler doesn't know that `'a` and `'b` are same, it refuses our code.

#### Another example

It's so fun to messing up with the lifetime checker, isn't it? I'll show you another Rust code.

```rust, line_num
fn foo(mut v: Vec<i32>) {
    let a = &mut v;
    let b = &mut v;
}
fn bar(mut v: Vec<i32>) {
    let c = &mut v;
    let d = &mut v;
    println!("{:?}", c);
}
fn baz(mut v: Vec<i32>) {
    let e = &mut v;
    let f = &mut v;
    println!("{:?}", f);
}
```

We've taught that there can exist only one mutable reference of a value. So, it seems that the lifetime checker will refuse all the 3 functions above... does it?

```
error[E0499]: cannot borrow `v` as mutable more than once at a time
 --> src\main.rs:7:13
  |
6 |     let c = &mut v;
  |             ------ first mutable borrow occurs here
7 |     let d = &mut v;
  |             ^^^^^^ second mutable borrow occurs here
8 |     println!("{:?}", c);
  |                      - first borrow later used here

For more information about this error, try `rustc --explain E0499`.
```

It only refuses `bar`. That's because *lifetime* is a very flexible concept. `a` and `e` are not used at all. So their lifetime ends as soon as they're declared. In those cases, their lifetime do not overlap with `b` and `f`'s lifetime. So it's fine to create another mutable reference. But in `c`'s case, it's lifetime ends at line 9, which is after `d` is declared.

See how it's visualized.

```rust, line_num
fn foo(mut v: Vec<i32>) {
    let a = &mut v;  // ---'a
    let b = &mut v;  // ---'b
}
fn bar(mut v: Vec<i32>) {
    let c = &mut v;       // --------+
    let d = &mut v;       // ---'d   |    -> Oops!
    println!("{:?}", c);  // --------+-'c
}
fn baz(mut v: Vec<i32>) {
    let e = &mut v;       // ---'e
    let f = &mut v;       // --+
    println!("{:?}", f);  // --+---'f
}
```

### Lifetime overlapping

In some cases, lifetimes may overlap. The compiler is smart enough to figure out whether the overlapping is safe. Let's look at the example below.

```rust, line_num
fn foo(mut v: Vec<i32>) {
    let v2 = &mut v;      // ------------+--'a
    let v3 = &mut v2[1];  // ---+--'b    |
    *v3 = 3;              // ---+        |
    v2.push(12);          // ------------+
}
fn bar(mut v: Vec<i32>) {
    let v2 = &mut v;      // -----+--'c
    let v3 = &mut v2[1];  // ----------+--'d
    v2.push(12);          // -----+    |
    *v3 = 3;              // ----------+
}
fn baz(mut v: Vec<i32>) {
    let v2 = &mut v;    // ------------+--'e
    let v3 = &mut *v2;  // ---+--'f    |
    v3[1] = 3;          // ---+        |
    v2.push(12);        // ------------+
}
fn biz(mut v: Vec<i32>) {
    let v2 = &mut v;    // ------------+--'g
    let v3 = &mut *v2;  // ---+--'h    |
    v2.push(12);        // ------------+
    v3[1] = 3;          // ---+
}
```

```
error[E0499]: cannot borrow `*v2` as mutable more than once at a time
  --> src\main.rs:10:5
   |
9  |     let v3 = &mut v2[1];
   |                   -- first mutable borrow occurs here
10 |     v2.push(12);
   |     ^^^^^^^^^^^ second mutable borrow occurs here
11 |     *v3 = 3;
   |     ------- first borrow later used here

error[E0499]: cannot borrow `*v2` as mutable more than once at a time
  --> src\main.rs:22:5
   |
21 |     let v3 = &mut *v2;
   |              -------- first mutable borrow occurs here
22 |     v2.push(12);
   |     ^^^^^^^^^^^ second mutable borrow occurs here
23 |     v3[1] = 3;
   |     -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `playground` due to 2 previous errors
```

`bar` and `biz` are refused due to the overlapping lifetimes, `foo` and `baz` are accepted. It's because *reborrowing* a mutable reference is allowed. It's fine if the lifetime of the re-borrowed reference ends before the referent is used.

One thing to note is that if we replace `v2` with `v` in line 3, 9, 15 and 21, every function will fail. It never allows multiple mutable reference of the same value.