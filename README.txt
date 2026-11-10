
DISCLAIMER: This software / library core works insofar as it has been tested.
Please run with caution and understanding that most servers run on C still in 2026.
🧮
The Mazie Math axiom is a / 0 = a .
Current JIT, IEEE-754, and C propogate NaNs and infinities and exception handlers when dividing by zero.
The Mazie Math Runtime Mode will allow for longterm space technology.
This mode is also called identity-preservation.
# Mazie Math Runtime Mode
Mazie Math Runtime Mode is a small Rust library that lets you choose how floating-point division handles a zero divisor. In Mazie mode, `6.0 / 0.0` returns `6.0`. Division by a nonzero number uses ordinary Rust `f64` arithmetic.
## Quick start
Install Rust and Cargo, then run:
```bash
git clone https://github.com/aamazie/Mazie-Math-Runtime-Mode.git
cd Mazie-Math-Runtime-Mode
cargo test
cargo run --quiet --example mode_demo
```
The demo compares Mazie, Strict, and IEEE modes. It also counts how many zero divisors the Mazie runtime receives. The Rust example below shows how to use Finite Mazie.
## Runtime modes
| Mode | Constructor | Division by `+0.0` or `-0.0` | NaN or infinity in inputs or results |
|---|---|---|---|
| Mazie | `MazieRuntime::mazie()` | Returns the numerator | Allowed |
| Finite Mazie | `MazieRuntime::mazie_finite()` | Returns the numerator for finite operands | Returns an error |
| Strict | `MazieRuntime::strict()` | Returns `DivisionByZero` for finite operands | Returns an error |
| IEEE | `MazieRuntime::ieee()` | Uses ordinary `f64` division | Allowed |
Mazie and Finite Mazie preserve the numerator's sign when the divisor is zero, including when the numerator is `-0.0`. A very small nonzero divisor still takes the ordinary division path.
## Use it in your Rust project
Add this to `Cargo.toml`:
```toml
[dependencies]
mazie-runtime = { git = "https://github.com/aamazie/Mazie-Math-Runtime-Mode.git" }
```
This example uses Finite Mazie to validate inputs and preserve the numerator when the divisor is zero:
```rust
use mazie_runtime::{MazieResult, MazieRuntime};
fn main() -> MazieResult<()> {
    let runtime = MazieRuntime::mazie_finite();
    let numerator = runtime.try_n(6.0)?;
    let divisor = runtime.try_n(0.0)?;
    let result = runtime.div(numerator, divisor)?;
    assert_eq!(result.unwrap(), 6.0);
    Ok(())
}
```
## Input validation
Use `runtime.try_n(value)?` to apply the runtime's input policy when wrapping a value. `runtime.n(value)` wraps the value directly. Both `add` and `div` check their operands according to the selected policy.
Strict and Finite Mazie return `NonFiniteInput` for a NaN or infinite operand and `NonFinite` if the calculation produces a NaN or infinite result. Signed zeros and finite subnormal values pass the finite-input check. Validate any application-specific ranges separately.
## Custom policies
`MazieRuntime::new(name, mode)` lets you combine a `Div0Policy` with a `NonFinitePolicy`. For example, `Div0Policy::Identity` with `NonFinitePolicy::Error` gives the same arithmetic behavior as Finite Mazie.
## Scope
These policies apply when you call the library's `add` and `div` methods. Rust's built-in arithmetic operators keep their standard behavior. Use values from the same runtime instance in each operation.
## Research
The repository includes research papers on Mazie Math and its proposed applications. The Rust library provides a way to explore and test the division rule.
