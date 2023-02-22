---
date: [2023, 2, 18]
tags: [rust]
preview: Reference-counting pointers are one of the most widely used smart pointers. It dynamically tracks the lifetime of an object, and decides when to destruct the item.
---

# Reference-Counting Pointers in Rust

Reference-counting pointers are one of the most widely used smart pointers. It dynamically tracks the lifetime of an object, and decides when to destruct the item. It seems like Rust doesn't need such thing. Rust is known for compile time lifetime checking, isn't it? But unfortunately, Rust does need dynamic memory management. There are some cases where compiler can never know when to free an object. Or there are cases where it's much simpler to use Reference counters than to deal with borrow checker's restrictions.

So, how does Rust implement RC pointers? There are two major obstacles in it.

First, we can't decide whom to own the value. If everyone has the copy of the data, that would be too expensive. We want only one copy of the data to exist, and the others should clone the pointer. The easiest way is to give the ownership to the one who calls `Rc::new()`, then give the pointer to the value to the others. But in that case, the value will be freed when the first owner dies. We don't want that. That's not what `Rc` is for.

Second, how do owners mutate the reference count? Rust allows at most one mutable reference to exist per a data. But there must be multiple owners of an `Rc`, who wants to mutate the counter.

The second one is easy, and the first one is quite tricky. Let's look at the implementation.

[std doc](https://doc.rust-lang.org/std/rc/struct.Rc.html)

```rust
pub struct Rc<T: ?Sized> {
    ptr: NonNull<RcBox<T>>,
    phantom: PhantomData<RcBox<T>>,
}
```

It's how Rust implements `Rc`. Though it's only a few lines, there are so many scary-looking names in the code. `Sized` and `PhantomData` are very important Rust concepts, but that's not what this article is for. We're not gonna take a closer look at them.

Let's first look at `RcBox`, which is an internal data wrapper of `Rc`. `RcBox` looks like below.

```rust
struct RcBox<T: ?Sized> {
    strong: Cell<usize>,
    weak: Cell<usize>,
    value: T,
}
```

`Rc` solves the second problem by using the `Cell` type. It implements an internal mutability. It's a special type that lets you set/get a data with a read-only reference (`&`). In order to do that, we have to break some basic rules of the language.

The Rust compiler prevents dangling pointers and double freeing with the [lifetime checker](Lifetime-In-Rust.html). Since the internal mutability stands against the checker, it has some restrictions. The `Cell` type never lets you own the pointer to the value. It moves the data in and out when accessing. It always copies the data.

The compiler assumes that the value behind `&` never changes. It does lots of optimizations based on the assumption. Mutating the value behind them may corrupt the optimizations, breaking the entire program. So, the language designers marked the type, telling the compiler that this type implements internal mutabilities. It disables some optimizations.

In order to solve the first problem, `Rc` uses a special type called `NonNull`. Basically, it's a raw-pointer. As it's name suggests, it must not be `null`. Does using raw pointers solve the problem? Who owns pointers, then? It's still difficult to decide how to call the destructor. In order to implement `Rc`, we have to disable the compiler's auto-destructing. The compiler's lifetime checker tracks the lifetime of *ALL* the objects in the code. All the destructor calls are added by the compiler. But, the lifetime of an `Rc` cannot be known at compile-time. So, we have to disable the lifetime checker manually. Below is how we do that.

```rust
impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        unsafe {
            Self::from_inner(
                Box::leak(Box::new(RcBox { strong: Cell::new(1), weak: Cell::new(1), value }))
                    .into(),
            )
        }
    }
}

impl<T: ?Sized> Rc<T> {
    unsafe fn from_inner(ptr: NonNull<RcBox<T>>) -> Self {
        Self { ptr, phantom: PhantomData }
    }
}
```

`Box::leak` disables the lifetime checker. It literally causes a memory leak. Once it's leaked, the `Rc` has to call the destructor of it's value manually when the reference-count is 0. Below is how it does that.

```rust, line_num
unsafe impl<#[may_dangle] T: ?Sized> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            self.inner().dec_strong();
            if self.inner().strong() == 0 {
                // destroy the contained object
                ptr::drop_in_place(Self::get_mut_unchecked(self));

                // remove the implicit "strong weak" pointer now that we've
                // destroyed the contents.
                self.inner().dec_weak();

                if self.inner().weak() == 0 {
                    Global.deallocate(self.ptr.cast(), Layout::for_value(self.ptr.as_ref()));
                }
            }
        }
    }
}
```

When an `Rc` is dropped, it checks whether the counter is 0. If so, it destroys the contained object by calling `ptr::drop_in_place`. You've probably never seen it before, because the compiler does it for us usually.

I also wanted to dig deeper by reading the source of `ptr::drop_in_place` and `Global.deallocate`, but it was no use. Those are compiler-builtins.

It's very interesting to look at the unsafeness inside safe APIs. I'll be back with more Rust std stuffs.
