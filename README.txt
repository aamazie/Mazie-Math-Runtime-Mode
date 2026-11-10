DISCLAIMER: This software / library core works insofar as it has been tested. 

Please run with caution and understanding that most servers run on C still in 2026.

🧮 

The Mazie Math axiom is a / 0 = a .

Current JIT, IEEE-754, and C propogate NaNs and infinities and exception handlers when dividing by zero.

The Mazie Math Runtime Mode will allow for longterm space technology.

This mode is also called identity-preservation.


# Mazie Math Runtime Mode
Mazie Runtime is a small Rust library that lets a program choose how floating-point division by zero behaves.
In **Mazie mode**, dividing a number by exact zero returns the numerator:
```text
6 / 0 = 6
```
This is a chosen rule for the runtime. Division by a nonzero number still uses ordinary `f64` arithmetic.
## Set up
Install Rust and Cargo, then clone the repository and run its tests:
```bash
git clone https://github.com/aamazie/Mazie-Math-Runtime-Mode.git
cd Mazie-Math-Runtime-Mode
cargo test
```
To use the library in another Rust project, add this to that project's `Cargo.toml`:
```toml
[dependencies]
mazie-runtime = { git = "https://github.com/aamazie/Mazie-Math-Runtime-Mode.git" }
```
## Basic example
```rust
use mazie_runtime::{MazieResult, MazieRuntime};
fn main() -> MazieResult<()> {
    let runtime = MazieRuntime::mazie();
    let six = runtime.n(6.0);
    let zero = runtime.n(0.0);
    let result = runtime.div(six, zero)?;
    assert_eq!(result.unwrap(), 6.0);
    Ok(())
}
```
Create numbers with `runtime.n(value)`, then call `runtime.add(...)` or `runtime.div(...)`. These operations return a `Result`. Call `unwrap()` on a successful `MazieNum` result to get its `f64` value.
## Modes
| Mode | Create it with | Division by exact zero | NaN or infinity from arithmetic |
|---|---|---|---|
| Mazie | `MazieRuntime::mazie()` | Returns the numerator | Allowed |
| Strict | `MazieRuntime::strict()` | Returns an error | Returns an error |
| IEEE | `MazieRuntime::ieee()` | Uses ordinary `f64` division | Allowed |
For a numerator of `6.0`:
```text
Divisor       Mazie mode    Strict mode       IEEE mode
+0.0          6.0           Error             +infinity
-0.0          6.0           Error             -infinity
2.0           3.0           3.0               3.0
```
Both `+0.0` and `-0.0` count as zero when the runtime selects a division policy. Mazie mode returns the numerator unchanged, including the sign of a `-0.0` numerator. A very small **nonzero** divisor takes the ordinary division path.
Strict mode reports `DivisionByZero` for a zero divisor. It reports `NonFinite` if addition or division produces NaN or infinity.
## Choose your own policy
You can combine a division rule with a rule for non-finite results:
```rust
use mazie_runtime::{Div0Policy, MazieMode, MazieRuntime, NonFinitePolicy};
let runtime = MazieRuntime::new(
    "my-runtime",
    MazieMode {
        div0: Div0Policy::Identity,
        non_finite: NonFinitePolicy::Error,
    },
);
```
This example returns the numerator for division by zero and reports an error if the result is NaN or infinity.
## Scope
The library applies these rules when code calls its `add` and `div` methods. It does not change Rust's arithmetic operators or arithmetic elsewhere in the system. Values belong to the runtime instance that created them, and operations reject values from different instances.
The Mazie rule handles the zero-divisor case. Other operations can still produce NaN or infinity when the selected mode allows them. Use strict mode or a custom policy if those results should be errors.
## Research
The repository also contains papers discussing Mazie Math and its proposed applications. The Rust code provides an implementation for exploring and testing the division rule.

To run a demo after testing, run:

cargo run --quiet --example mode_demo
