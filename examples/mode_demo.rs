use std::sync::Arc;

use mazie_runtime::{Div0Counter, MazieError, MazieNum, MazieResult, MazieRuntime};

fn display(result: MazieResult<MazieNum>) -> String {
    match result {
        Ok(number) => {
            let value = number.unwrap();
            if value == f64::INFINITY {
                return "+inf".to_owned();
            }
            format!("{value:.1}")
        }
        Err(MazieError::DivisionByZero { .. }) => "DivisionByZero".to_owned(),
        Err(error) => error.to_string(),
    }
}

fn main() {
    let counter = Arc::new(Div0Counter::new());
    let modes = [
        MazieRuntime::mazie().with_hook(counter.clone()),
        MazieRuntime::strict(),
        MazieRuntime::ieee(),
    ];

    println!("Mazie Math Runtime Mode");
    println!();
    println!("{:<14} {:>10} {:>16} {:>8}", "Input", "Mazie", "Strict", "IEEE");

    for (numerator, divisor) in [(6.0, 2.0), (6.0, 0.0), (6.0, -0.0), (0.0, 0.0)] {
        let input = format!("{numerator:.1} / {divisor:.1}");
        let [mazie, strict, ieee] = modes
            .each_ref()
            .map(|runtime| display(runtime.div(runtime.n(numerator), runtime.n(divisor))));
        println!("{input:<14} {mazie:>10} {strict:>16} {ieee:>8}");
    }

    println!();
    println!("Zero divisors recorded by the Mazie runtime: {}", counter.load());
}
