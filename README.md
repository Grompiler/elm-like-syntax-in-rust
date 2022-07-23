# elm-like-syntax-in-rust
define simple functions just like in elm

### example
```rust
fn main() {
    elm!(
        identity: i32 -> i32
        identity a =
            a
    );
    let identity = identity(10);

    println!("{identity}");
}
```
> 10
