# 보고서

[[right]]
[[small]]2018-14566 배현솔[[/small]]
[[/right]]

## Sample h2

This is a sample report.

I would be using a lot of lists when writing reports.

- Elem1
  - Elem1.1
  - Elem1.2
- Elem2
- Elem3

```rust,line_num,copy_button(false)
fn qs(arr: &Vec<u32>) -> Vec<u32> {

    if arr.len() < 2 {
        return arr.clone();
    }

    let pivot = arr[0];

    vec![
        qs(arr.iter().filter(|n| n < pivot).map(|n| **n).collect::<Vec<u32>>()),
        arr.iter().filter(|n| n == pivot).map(|n| **n).collect::<Vec<u32>>(),
        qs(arr.iter().filter(|n| n < pivot).map(|n| **n).collect::<Vec<u32>>())
    ].concat()
}
```

## qtxvc123

|a|v|
|-|-|
|!![[collapsible]]|
|2|3asdf|

```rust
println!("Hello World!");
println!("Hello World!");
println!("Hello World!");
println!("Hello World!");
```

## exex

[123](#qtxvc123)