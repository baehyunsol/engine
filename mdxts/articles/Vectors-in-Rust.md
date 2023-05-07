---
date: [2023, 5, 7]
tags: [rust, smart-pointers]
preview: It's the second article of the Rust smart pointers series. This time, it's vectors. If you're writting code in Rust, there's no way you can avoid using this type. Knowing the internals of one of the most frequently used type will improve our code a lot!
---

# Vectors in Rust

It's the second article of the Rust smart pointers series. This time, it's vectors. If you're writting code in Rust, there's no way you can avoid using this type. Knowing the internals of one of the most frequently used type will improve our code a lot!

[[box]]

All the std code in this article is from the lastest Rust repo as I'm writting this article (07-May-2023). The implementation may change over time.

[[/box]]

## Definition

```rust, line_num
#[stable(feature = "rust1", since = "1.0.0")]
#[cfg_attr(not(test), rustc_diagnostic_item = "Vec")]
#[rustc_insignificant_dtor]
pub struct Vec<T, #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

Above is the definition of `Vec<T>`. Ignoring the macros for debug information, it consists of a buffer and a length. The buffer has type `RawVec<T>`, which looks like below.

```rust, line_num
#[allow(missing_debug_implementations)]
pub(crate) struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    alloc: A,
}
```

`RawVec` has 3 fields: pointer to the actual data, capacity of the buffer, and a memory allocator. [`Unique`](#uniqueptr) is a speical type of raw pointer.

A vector is very simple. It has a pointer to a buffer, a length and a capacity. Just like what we saw in Data Structure courses.

## Initialization of a vector

What happens when we initialize a vector? The most common way to initialize a vector is using `vec!`. `vec!` is defined like below.

```rust, line_num
#[cfg(all(not(no_global_oom_handling), not(test)))]
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "vec_macro"]
#[allow_internal_unstable(rustc_attrs, liballoc_internals)]
macro_rules! vec {
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!(<[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            #[rustc_box]
            $crate::boxed::Box::new([$($x),+])
        ))
    );
}
```

There are 3 cases of a vector initialization. If a programmer wants an empty vector, it calls [`vec::Vec::new()`](#vecvecnew). It guarantees that it doesn't allocate any memory when an empty vector is initialized. If a programmer initializes vector like `vec![x; 100]`, it calls [`vec::from_elem`](#vecfromelem). Click the link to see the explanation of the function.

For the last branch, which is the most common way to define a vector, wraps a buffer with `Box` and calls `into_vec`.

```rust, line_num
// library/alloc/src/slice.rs
pub fn into_vec<T, A: Allocator>(b: Box<[T], A>) -> Vec<T, A> {
    unsafe {
        let len = b.len();
        let (b, alloc) = Box::into_raw_with_allocator(b);
        Vec::from_raw_parts_in(b as *mut T, len, len, alloc)
    }
}

// library/alloc/src/vec/mod.rs
impl<T, A: Allocator> Vec<T, A> {
    #[inline]
    #[unstable(feature = "allocator_api", issue = "32838")]
    pub unsafe fn from_raw_parts_in(ptr: *mut T, length: usize, capacity: usize, alloc: A) -> Self {
        unsafe { Vec { buf: RawVec::from_raw_parts_in(ptr, capacity, alloc), len: length } }
    }
}
```

`into_vec` extracts information needed for the initialization of a vector, then calls `Vec::from_raw_parts_in`. An interesting part here is that the capacity and the length are the same. It doesn't allocate additional space for future push operations.

## Read an element of a vector

How does it access an element of a vector? It goes through quite complicated steps. Below are some important functions in the steps. The functions below are called from top to bottom.

```rust, line_num
// library/alloc/src/vec/mod.rs
impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}

// core/src/slice/index.rs
#[stable(feature = "slice_get_slice_impls", since = "1.15.0")]
#[rustc_const_unstable(feature = "const_slice_index", issue = "none")]
unsafe impl<T> SliceIndex<[T]> for usize {
    type Output = T;

    #[inline]
    fn get(self, slice: &[T]) -> Option<&T> {
        // SAFETY: `self` is checked to be in bounds.
        if self < slice.len() { unsafe { Some(&*self.get_unchecked(slice)) } } else { None }
    }

    #[inline]
    unsafe fn get_unchecked(self, slice: *const [T]) -> *const T {
        let this = self;
        // SAFETY: the caller guarantees that `slice` is not dangling, so it
        // cannot be longer than `isize::MAX`. They also guarantee that
        // `self` is in bounds of `slice` so `self` cannot overflow an `isize`,
        // so the call to `add` is safe.
        unsafe {
            assert_unsafe_precondition!(
                "slice::get_unchecked requires that the index is within the slice",
                [T](this: usize, slice: *const [T]) => this < slice.len()
            );
            slice.as_ptr().add(self)
        }
    }

}
```

It first checks whether the index is in bounds. If so, it returns the pointer of the desired element by adding the index to the pointer of the buffer.

## Push an element to a vector

Pushing an element is more complex than reading/initializing because it may change the metadata of a vector. Let's see how `.push` is defined.

```rust, line_num
#[cfg(not(no_global_oom_handling))]
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
pub fn push(&mut self, value: T) {
    // This will panic or abort if we would allocate > isize::MAX bytes
    // or if the length increment would overflow for zero-sized types.
    if self.len == self.buf.capacity() {
        self.buf.reserve_for_push(self.len);
    }
    unsafe {
        let end = self.as_mut_ptr().add(self.len);
        ptr::write(end, value);
        self.len += 1;
    }
}
```

The `push` method itself is very simple. It checks whether the capacity is available, then writes a new value at the end of the buffer. If there's no enough capacity, it calls `.reserve_for_push`. It's what we're interested in.

```rust, line_num
impl<T, A: Allocator> RawVec<T, A> {

    pub(crate) const MIN_NON_ZERO_CAP: usize = if mem::size_of::<T>() == 1 {
        8
    } else if mem::size_of::<T>() <= 1024 {
        4
    } else {
        1
    };

    /// A specialized version of `reserve()` used only by the hot and
    /// oft-instantiated `Vec::push()`, which does its own capacity check.
    #[cfg(not(no_global_oom_handling))]
    #[inline(never)]
    pub fn reserve_for_push(&mut self, len: usize) {
        handle_reserve(self.grow_amortized(len, 1));
    }

}

impl<T, A: Allocator> RawVec<T, A> {

    fn set_ptr_and_cap(&mut self, ptr: NonNull<[u8]>, cap: usize) {
        // Allocators currently return a `NonNull<[u8]>` whose length matches
        // the size requested. If that ever changes, the capacity here should
        // change to `ptr.len() / mem::size_of::<T>()`.
        self.ptr = unsafe { Unique::new_unchecked(ptr.cast().as_ptr()) };
        self.cap = cap;
    }

    // This method is usually instantiated many times. So we want it to be as
    // small as possible, to improve compile times. But we also want as much of
    // its contents to be statically computable as possible, to make the
    // generated code run faster. Therefore, this method is carefully written
    // so that all of the code that depends on `T` is within it, while as much
    // of the code that doesn't depend on `T` as possible is in functions that
    // are non-generic over `T`.
    fn grow_amortized(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        // This is ensured by the calling contexts.
        debug_assert!(additional > 0);

        if T::IS_ZST {
            // Since we return a capacity of `usize::MAX` when `elem_size` is
            // 0, getting to here necessarily means the `RawVec` is overfull.
            return Err(CapacityOverflow.into());
        }

        // Nothing we can really do about these checks, sadly.
        let required_cap = len.checked_add(additional).ok_or(CapacityOverflow)?;

        // This guarantees exponential growth. The doubling cannot overflow
        // because `cap <= isize::MAX` and the type of `cap` is `usize`.
        let cap = cmp::max(self.cap * 2, required_cap);
        let cap = cmp::max(Self::MIN_NON_ZERO_CAP, cap);

        let new_layout = Layout::array::<T>(cap);

        // `finish_grow` is non-generic over `T`.
        let ptr = finish_grow(new_layout, self.current_memory(), &mut self.alloc)?;
        self.set_ptr_and_cap(ptr, cap);
        Ok(())
    }

}

#[inline(never)]
fn finish_grow<A>(
    new_layout: Result<Layout, LayoutError>,
    current_memory: Option<(NonNull<u8>, Layout)>,
    alloc: &mut A,
) -> Result<NonNull<[u8]>, TryReserveError>
where
    A: Allocator,
{
    // Check for the error here to minimize the size of `RawVec::grow_*`.
    let new_layout = new_layout.map_err(|_| CapacityOverflow)?;

    alloc_guard(new_layout.size())?;

    let memory = if let Some((ptr, old_layout)) = current_memory {
        debug_assert_eq!(old_layout.align(), new_layout.align());
        unsafe {
            // The allocator checks for alignment equality
            intrinsics::assume(old_layout.align() == new_layout.align());
            alloc.grow(ptr, old_layout, new_layout)
        }
    } else {
        alloc.allocate(new_layout)
    };

    memory.map_err(|_| AllocError { layout: new_layout, non_exhaustive: () }.into())
}

// Central function for reserve error handling.
#[cfg(not(no_global_oom_handling))]
#[inline]
fn handle_reserve(result: Result<(), TryReserveError>) {
    match result.map_err(|e| e.kind()) {
        Err(CapacityOverflow) => capacity_overflow(),
        Err(AllocError { layout, .. }) => handle_alloc_error(layout),
        Ok(()) => { /* yay */ }
    }
}
```

Well, that's quite long. When a buffer doesn't have any more space, it calls `reserve_for_push` to grow the size of the vector. It first calls `grow_amortized`, which determines the capacity of the new vector, and allocates the actual memory. `handle_reserve` checks whether the allocation was successful.

`grow_amortized` takes 2 arguments: current length and the number of additional elements of the vector. For our `push` case, `additional` is 1. It first tries to double the size of the buffer. If the doubled buffer is still not big enough, it makes the buffer size exactly `len + additional`. If the new capacity is smaller than the minial required capacity, it grows the capacity again. `MIN_NON_ZERO_CAP` is defined at line 3 above. It's 8 for byte-sized types, 4 for small types and 1 for big ones. `finish_grow` actually allocates the new memory, and makes sure that the newly allocated memory is valid.

[[box]]

In case you wonder what the line 42 means...

For 0-sized types, it doesn't allocate any space. It sets capacity to be `usize::MAX` for 0-sized types. Since this function is called only when `length >= capacity`, line 42 is true only when the programmer tries to push more than `usize::MAX` elements to a vector. It returns `Err(CapacityOverflow)` in such case.

[[/box]]

## For Rust programmers

Above sections contain some useful information for Rust programmers. First, when we initialize a vector using `vec!`, it doesn't allocate additional space. The length and the capacity are the same. When we push an element, a vector is guaranteed to have a capacity of at least 4 elements (for most types). If the capacity is to grow, it's doubled.

## Appendix

### `vec::Vec::new()`

[[anchor, id = vec Vec new]][[/anchor]]

```rust, line_num
#[inline]
#[rustc_const_stable(feature = "const_vec_new", since = "1.39.0")]
#[stable(feature = "rust1", since = "1.0.0")]
#[must_use]
pub const fn new() -> Self {
    Vec { buf: RawVec::NEW, len: 0 }
}
```

Below is the definition of `RawVec::NEW`.

```rust, line_num
impl<T> RawVec<T, Global> {
    /// HACK(Centril): This exists because stable `const fn` can only call stable `const fn`, so
    /// they cannot call `Self::new()`.
    ///
    /// If you change `RawVec<T>::new` or dependencies, please take care to not introduce anything
    /// that would truly const-call something unstable.
    pub const NEW: Self = Self::new();

    /// Creates the biggest possible `RawVec` (on the system heap)
    /// without allocating. If `T` has positive size, then this makes a
    /// `RawVec` with capacity `0`. If `T` is zero-sized, then it makes a
    /// `RawVec` with capacity `usize::MAX`. Useful for implementing
    /// delayed allocation.
    #[must_use]
    pub const fn new() -> Self {
        Self::new_in(Global)
    }
}
```

As the comment says, it guarantees that `Vec::new()` doesn't allocate any memory.

### `vec::from_elem()`

[[anchor, id = vec from_elem]][[/anchor]]

```rust, line_num
impl<T: Clone> SpecFromElem for T {
    default fn from_elem<A: Allocator>(elem: Self, n: usize, alloc: A) -> Vec<Self, A> {
        let mut v = Vec::with_capacity_in(n, alloc);
        v.extend_with(n, ExtendElement(elem));
        v
    }
}
```

It calls `.extend_with` with the element and `n`. `ExtendElement` is a generator that endlessly clones `elem`. That's why `T` has to satisfy `Clone`.

```rust, line_num
impl<T, A: Allocator> Vec<T, A> {
    #[cfg(not(no_global_oom_handling))]
    /// Extend the vector by `n` values, using the given generator.
    fn extend_with<E: ExtendWith<T>>(&mut self, n: usize, mut value: E) {
        self.reserve(n);

        unsafe {
            let mut ptr = self.as_mut_ptr().add(self.len());
            // Use SetLenOnDrop to work around bug where compiler
            // might not realize the store through `ptr` through self.set_len()
            // don't alias.
            let mut local_len = SetLenOnDrop::new(&mut self.len);

            // Write all elements except the last one
            for _ in 1..n {
                ptr::write(ptr, value.next());
                ptr = ptr.add(1);
                // Increment the length in every step in case next() panics
                local_len.increment_len(1);
            }

            if n > 0 {
                // We can write the last element directly without cloning needlessly
                ptr::write(ptr, value.last());
                local_len.increment_len(1);
            }

            // len set by scope guard
        }
    }
}
```

As mentioned above, `ExtendWith` is an endless generator. `.next()` at line 16 clones `value`.

### `Unique`

[[anchor, id = unique ptr]][[/anchor]]

The explanation below is from the comment in the compiler source code.

[[box]]

```rust, line_num
#[unstable(
    feature = "ptr_internals",
    issue = "none",
    reason = "use `NonNull` instead and consider `PhantomData<T>` \
              (if you also use `#[may_dangle]`), `Send`, and/or `Sync`"
)]
#[doc(hidden)]
#[repr(transparent)]
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    // NOTE: this marker has no consequences for variance, but is necessary
    // for dropck to understand that we logically own a `T`.
    //
    // For details, see:
    // https://github.com/rust-lang/rfcs/blob/master/text/0769-sound-generic-drop.md#phantom-data
    _marker: PhantomData<T>,
}
```

A wrapper around a raw non-null `*mut T` that indicates that the possessor of this wrapper owns the referent. Useful for building abstractions like `Box<T>`, `Vec<T>`, `String`, and `HashMap<K, V>`.

Unlike `*mut T`, `Unique<T>` behaves "as if" it were an instance of `T`. It implements `Send`/`Sync` if `T` is `Send`/`Sync`. It also implies the kind of strong aliasing guarantees an instance of `T` can expect: the referent of the pointer should not be modified without a unique path to its owning Unique.

If you're uncertain of whether it's correct to use `Unique` for your purposes, consider using `NonNull`, which has weaker semantics.

Unlike `*mut T`, the pointer must always be non-null, even if the pointer is never dereferenced. This is so that enums may use this forbidden value as a discriminant -- `Option<Unique<T>>` has the same size as `Unique<T>`. However the pointer may still dangle if it isn't dereferenced.

Unlike `*mut T`, `Unique<T>` is covariant over `T`. This should always be correct for any type which upholds Unique's aliasing requirements.

[[/box]]