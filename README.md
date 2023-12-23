# Dwat20
Dwat20 is a Rust crate to emulate dice rolls for my projects.

## Planned Features
Features will be added to this section as they come up in my other projects.

- Roll can return success or failure.
- Roll can return total value of multiple dice.
- Roll can return number of successes of multiple dice.
- Augment roll with modifiers.
- Option to explode dice rolls.
- Option to use Vectors for representing side faces.

## Documentation
To install Dwat20 run:
```shell
cargo install dwat20
``` 

### Examples
A simple d20:
```rust
use dwat20::*;

fn main() {
    let die = Die::new(20);

    println!("Die result: {}", die.roll())
}
```
Output:
```
Die result: 7
```