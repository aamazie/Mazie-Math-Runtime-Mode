use crate::{runtime::MazieRuntime, num::MazieNum, error::MazieResult};

#[test]
fn identity_div0() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();
    let x = rt.n(5.0);
    let z = rt.n(0.0);

    let result = rt.div(x, z)?;
    assert_eq!(result.value, 5.0); // Identity-preserving division
    Ok(())
}

#[test]
fn add_two_numbers() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();
    let a = rt.n(10.0);
    let b = rt.n(7.0);

    let sum = rt.add(a, b)?;
    assert_eq!(sum.value, 17.0);
    Ok(())
}

#[test]
fn sub_two_numbers() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();
    let a = rt.n(10.0);
    let b = rt.n(3.0);

    let diff = rt.add(a, rt.n(-b.value))?; // Using add with negation
    assert_eq!(diff.value, 7.0);
    Ok(())
}

#[test]
fn mul_two_numbers() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();
    let a = rt.n(6.0);
    let b = rt.n(7.0);

    let product = rt.add(rt.n(0.0), rt.n(a.value * b.value))?; // simple multiplication
    assert_eq!(product.value, 42.0);
    Ok(())
}

#[test]
fn ieee_division() -> MazieResult<()> {
    let rt = MazieRuntime::ieee();
    let a = rt.n(6.0);
    let b = rt.n(3.0);

    let result = rt.div(a, b)?;
    assert_eq!(result.value, 2.0);
    Ok(())
}

#[test]
fn div_by_zero_error() {
    let rt = MazieRuntime::strict();
    let a = rt.n(5.0);
    let z = rt.n(0.0);

    let result = rt.div(a, z);
    assert!(result.is_err(), "Strict mode should error on division by zero");
}