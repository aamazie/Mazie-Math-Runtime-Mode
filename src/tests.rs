use crate::{MazieError, MazieResult, MazieRuntime};

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

#[test]
fn mazie_mode_preserves_the_numerator_for_both_signed_zeros() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();

    for divisor in [f64::from(0_i32), 0.0, -0.0] {
        for numerator in [6.0, -6.0, 0.0, -0.0] {
            let result = rt.div(rt.n(numerator), rt.n(divisor))?.unwrap();

            // Bitwise comparison distinguishes +0.0 from -0.0.
            assert_eq!(result.to_bits(), numerator.to_bits());
        }
    }

    Ok(())
}

#[test]
fn mazie_mode_uses_ordinary_division_for_small_nonzero_divisors() -> MazieResult<()> {
    let rt = MazieRuntime::mazie();

    for divisor in [f64::MIN_POSITIVE, -f64::MIN_POSITIVE] {
        let result = rt.div(rt.n(6.0), rt.n(divisor))?.unwrap();
        assert_eq!(result, 6.0 / divisor);
    }

    Ok(())
}

#[test]
fn ieee_and_strict_modes_handle_both_signed_zeros() -> MazieResult<()> {
    let ieee = MazieRuntime::ieee();
    let strict = MazieRuntime::strict();

    for (divisor, expected) in [(0.0, f64::INFINITY), (-0.0, f64::NEG_INFINITY)] {
        assert_eq!(ieee.div(ieee.n(6.0), ieee.n(divisor))?.unwrap(), expected);
        assert!(ieee.div(ieee.n(0.0), ieee.n(divisor))?.unwrap().is_nan());

        assert!(matches!(
            strict.div(strict.n(6.0), strict.n(divisor)),
            Err(MazieError::DivisionByZero { .. })
        ));
    }

    Ok(())
}

#[test]
fn strict_mode_rejects_non_finite_results() {
    let rt = MazieRuntime::strict();
    let max = rt.n(f64::MAX);

    assert!(matches!(
        rt.add(max, max),
        Err(MazieError::NonFinite { op: "add", value, .. }) if value == f64::INFINITY
    ));
    assert!(matches!(
        rt.div(max, rt.n(0.5)),
        Err(MazieError::NonFinite { op: "div", value, .. }) if value == f64::INFINITY
    ));
}

#[test]
fn finite_modes_reject_non_finite_inputs() {
    for rt in [MazieRuntime::strict(), MazieRuntime::mazie_finite()] {
        for input in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(matches!(
                rt.try_n(input),
                Err(MazieError::NonFiniteInput { op: "try_n", .. })
            ));
            assert!(matches!(
                rt.div(rt.n(6.0), rt.n(input)),
                Err(MazieError::NonFiniteInput { op: "div", input: "right operand", .. })
            ));
            assert!(matches!(
                rt.div(rt.n(input), rt.n(0.0)),
                Err(MazieError::NonFiniteInput { op: "div", input: "left operand", .. })
            ));
            assert!(matches!(
                rt.add(rt.n(input), rt.n(1.0)),
                Err(MazieError::NonFiniteInput { op: "add", input: "left operand", .. })
            ));
        }
    }
}

#[test]
fn finite_mazie_preserves_identity_and_rejects_overflow() -> MazieResult<()> {
    let rt = MazieRuntime::mazie_finite();
    let numerator = rt.try_n(-0.0)?;
    for divisor in [0.0, -0.0] {
        assert_eq!(
            rt.div(numerator, rt.try_n(divisor)?)?.unwrap().to_bits(),
            (-0.0_f64).to_bits()
        );
    }
    assert!(matches!(
        rt.div(rt.try_n(f64::MAX)?, rt.try_n(0.5)?),
        Err(MazieError::NonFinite { op: "div", .. })
    ));
    Ok(())
}

#[test]
fn permissive_modes_allow_non_finite_inputs() -> MazieResult<()> {
    let mazie = MazieRuntime::mazie();
    assert!(mazie.div(mazie.try_n(f64::NAN)?, mazie.n(0.0))?.unwrap().is_nan());

    let ieee = MazieRuntime::ieee();
    assert_eq!(ieee.div(ieee.n(6.0), ieee.try_n(f64::INFINITY)?)?.unwrap(), 0.0);
    Ok(())
}
