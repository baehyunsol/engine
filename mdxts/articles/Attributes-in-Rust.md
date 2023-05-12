---
date: [2023, 5, 12]
tags: [rust]
preview: Have you used attributes in the Rust language? Maybe yes, but probably never defined one. Though you don't have to define attributes in most cases, sometimes new attributes give you a very powerful and flexible set of skills.
---

# Attributes in Rust

Have you used attributes in the Rust language? Maybe yes, but probably never defined one. Though you don't have to define attributes in most cases, sometimes new attributes can provide you with a powerful and flexible set of features. I've always been interested in learning how to define attributes and exploring their capabilities.

This post is about defining a new attribute, and see how powerful attributes are.

## Environment

In order to define a new attribute, you have to set `proc-macro` in `Cargo.toml`. Add below lines to your `Cargo.toml` file.

```toml
[lib]
proc-macro = true
```

We're going to define the attribute in `lib.rs`, and test it in `main.rs`. Make those files in `/src`

```
├ src
│ ├ lib.rs
│ └ main.rs
└ Cargo.toml
```

Attribute-related stuffs are defined in the `proc_macro` crate. Add below lines at the top of `lib.rs`.

```rust
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};
```

[TokenStream] and [TokenTree] are the main components of a definition of attributes. We'll look at those in later sections.

## How an attribute looks like

```rust
#[proc_macro_attribute]
pub fn attribute1(attr: TokenStream, item: TokenStream) -> TokenStream {
    // ... func body
}
```

An attribute is defined like above. [TokenStream] is literally a stream of tokens. The function takes tokens as inputs and outputs new tokens. The output tokens replace the original tokens in the code. Interesting point here is that the function is written in Rust. That means you can write a Rust code that directly handles the AST of another Rust code. Sounds very strong, doesn't it?

Let's see how [TokenStream] works.

```rust
// lib.rs

#[proc_macro_attribute]
pub fn attribute1(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {attr:?}\n\nitem: {item:?}");
    item
}

// main.rs

#[attribute1 (a, b)]
fn main() {
    println!("Hello World!");
}
```

I defined a very simple attribute that prints out the inputs and returns `item` without modifying it. When you add the attribute to the main function and run it, the result is like below. The output of the code is not formatted, but I did it manually for readability.

- The compiler outputs the result when you compile the code, not when you run it. You have to checkout the std-out after running `cargo build`.
- `(a, b)` after `attribute1` doesn't do anything. I added them to demonstrate how the attribute system works. You can add any tokens surrounded by delimiters ("()", "{}" and "[]").

```rust
attr: TokenStream [
    Ident { ident: "a", span: #0 bytes(196..197) },
    Punct { ch: ',', spacing: Alone, span: #0 bytes(197..198) },
    Ident { ident: "b", span: #0 bytes(199..200) }
]

item: TokenStream [
    Ident { ident: "fn", span: #0 bytes(203..205) },
    Ident { ident: "main", span: #0 bytes(206..210) },
    Group {
        delimiter: Parenthesis,
        stream: TokenStream [],
        span: #0 bytes(210..212)
    },
    Group {
        delimiter: Brace,
        stream: TokenStream [
            Ident { ident: "println", span: #0 bytes(219..226) },
            Punct { ch: '!', spacing: Alone, span: #0 bytes(226..227) },
            Group {
                delimiter: Parenthesis,
                stream: TokenStream [
                    Literal {
                        kind: Str,
                        symbol: "Hello World!",
                        suffix: None,
                        span: #0 bytes(228..242)
                    }
                ],
                span: #0 bytes(227..243)
            },
            Punct {
                ch: ';',
                spacing: Alone,
                span: #0 bytes(243..244)
            }
        ],
        span: #0 bytes(213..246)
    }
]
```

The first parameter of the function (`attr`) is the stream of the tokens following the attribute. The second one (`item`) contains all the tokens in the definition of the main function. It even includes keywords like "fn", and it wraps braces and parenthesises into a `Group` variant.

All you have to do is to gather information from [TokenStream]s and construct new one by modifying them (or from scratch).

In order to read the tokens, we use `into_iter` method. I'm not sure why, but it doesn't implement `iter` method. You should clone it if you don't want to consume it. The method returns an iterator of [TokenTree], which is an enum for Rust tokens. The enum has 4 variants. All the variants share the API for spans ([set_span](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html#method.set_span) and [span](https://doc.rust-lang.org/proc_macro/enum.TokenTree.html#method.span)). The methods sets or gets span, which is the line number and the column number of the token. It's useful if you want to output precise error messages. Let's look at the details of each variant.

```rust
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```

- [Group](https://doc.rust-lang.org/proc_macro/struct.Group.html)
- [Ident](https://doc.rust-lang.org/proc_macro/struct.Ident.html)
- [Punct](https://doc.rust-lang.org/proc_macro/struct.Punct.html)
- [Literal](https://doc.rust-lang.org/proc_macro/struct.Literal.html)

Click the links to see their documents.

`Group` is a [TokenStream] surrounded by `Delimiter`s. There are 4 types of `Delimiter`s, which are brackets `[]`, braces `{}`, parenthesis `()`, and none ` `. The last one is used to preserve operator priorities of macro expansions. `.stream()` method returns the contained [TokenStream], without the delimiters. We'll use it most often.

`Ident` is for an identifier. `.to_string()` turns it into a `String`, and `new(&str, Span)` creates new one.

`Literal` is for a literal, like `"Hello World!"` or `3.14`. `.to_string()` turns it into a `String`, and `string(&str)` creates a new one. There are methods for all kind of literals (`u32_suffixed`, `f64_unsuffixed`, `character`, ...).

`Punct` represents a single-character punctuation. For example, an AddAssign operator (`+=`) is translated to 2 `Punct`s, `+` and `=`. It uses a [Spacing](https://doc.rust-lang.org/proc_macro/enum.Spacing.html) enum to differentiate a multi-character token and a sequence of single-character tokens. `.as_char()` turns it into a `char`, and `.new(char, Spacing)` creates new one.

## Defining a new attribute

Let's actually define a practical attribute. We'll implement an attribute for functions. It makes a function print its name when called.

```rust
#[print_name]
fn foo(a: u32, b: u32) -> u32 {
    a + b
}

fn foo_with_attribute(a: u32, b: u32) -> u32 {
    println!("Hi! my name is foo");
    a + b
}
```

Above is our goal. `foo` with `#[print_name]` becomes `foo_with_attribute`. It prints its name at the first line of its body. I won't change the name of the function in the implementation.

```rust
#[proc_macro_attribute]
pub fn print_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = vec![];
    let mut curr_state = State::Init;
    let mut func_name = String::new();

    // todo: construct a new TokenStream

    result.into_iter().collect()
}
```

I'll use a finite state machine to read the stream. When a function is found, it adds a `println!` macro at the first line of its body.

The state machine has 3 states: `Init`, `FnInit` and `BodyInit`.

```rust
for token in item.clone().into_iter() {

    match curr_state {
        State::Init => match &token {
            // todo
        }
        State::FnInit => match &token {
            // todo
        }
        State::BodyInit => match &token {
            // todo
        }
    }

    result.push(token);
}
```

It's a very simple FSM. I cloned `item` because I want to return the original one if it's not a function.

```rust
State::Init => match &token {
    TokenTree::Ident(i) => {
        let i_string = i.to_string();

        if i_string == "fn" {
            curr_state = State::FnInit;
        }

        else if i_string == "struct" || i_string == "enum" || i_string == "union" || i_string == "type" {
            return item;
        }

    }
    _ => {}
}
```

The first state, `State::Init` waits for the keyword `fn`. If it figures out that it's not dealing with a function, it returns immediately.

```rust
State::FnInit => match &token {
    TokenTree::Ident(i) => {
        func_name = i.to_string();
        curr_state = State::BodyInit;
    }
    _ => {}
}
```

The second state reads the name of the function. It assumes that the identifier following `fn` is the name of the function.

```rust
State::BodyInit => match &token {
    TokenTree::Group(g) => match g.delimiter() {
        Delimiter::Brace => {
            let mut print_name = TokenStream::from_str(&format!("println!(\"Hi! my name is {func_name}\");")).unwrap();
            print_name.extend(g.stream());
            let new_group = Group::new(Delimiter::Brace, print_name);
            result.push(TokenTree::Group(new_group));

            continue;
        },
        _ => {}  // not a function body
    }
    _ => {}
}
```

The last state adds the print statement at the function body. It assumes that a `Delimiter::Brace` appears if and only if it meets the function body. It constructs a new `Group` instance with the print statement, pushes it to `result` and continue the loop immediately, so that it doesn't push the `Group` twice.

## Complete code

- `lib.rs`

```rust, line_num
extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Delimiter, Group};
use std::str::FromStr;

#[proc_macro_attribute]
pub fn print_name(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = vec![];
    let mut curr_state = State::Init;
    let mut func_name = String::new();

    for token in item.clone().into_iter() {

        match curr_state {
            State::Init => match &token {
                TokenTree::Ident(i) => {
                    let i_string = i.to_string();

                    if i_string == "fn" {
                        curr_state = State::FnInit;
                    }

                    else if i_string == "struct" || i_string == "enum" || i_string == "union" || i_string == "type" {
                        return item;
                    }

                }
                _ => {}
            }
            State::FnInit => match &token {
                TokenTree::Ident(i) => {
                    func_name = i.to_string();
                    curr_state = State::BodyInit;
                }
                _ => {}
            }
            State::BodyInit => match &token {
                TokenTree::Group(g) => match g.delimiter() {
                    Delimiter::Brace => {
                        let mut print_name = TokenStream::from_str(&format!("println!(\"Hi! my name is {func_name}\");")).unwrap();
                        print_name.extend(g.stream());
                        let new_group = Group::new(Delimiter::Brace, print_name);
                        result.push(TokenTree::Group(new_group));

                        continue;
                    },
                    _ => {}  // not a function body
                }
                _ => {}
            }
        }

        result.push(token);
    }

    result.into_iter().collect()
}

enum State {
    Init,
    FnInit,
    BodyInit
}
```

- `main.rs`

```rust
use attr_test::*;  // name of the library you defined

#[print_name]
fn foo(a: u32, b: u32) -> u32 {
    a + b - 1
}

fn main() {
    foo(100, 200);
}
```

[TokenStream]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
[TokenTree]: https://doc.rust-lang.org/proc_macro/enum.TokenTree.html