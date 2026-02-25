#[cfg(test)]
mod tests {
    use crate::MazieRuntime;

    #[test]
    fn identity_div0() {
        let rt = MazieRuntime::mazie();
        let x = rt.n(5.0);
        let z = rt.n(0.0);
        assert_eq!(rt.div(x, z).unwrap().unwrap(), 5.0);
    }

    #[test]
    fn strict_div0_errors() {
        let rt = MazieRuntime::strict();
        let x = rt.n(5.0);
        let z = rt.n(0.0);
        assert!(rt.div(x, z).is_err());
    }

    #[test]
    fn runtime_mismatch() {
        let a = MazieRuntime::mazie();
        let b = MazieRuntime::strict();
        let x = a.n(1.0);
        let y = b.n(2.0);
        assert!(a.add(x, y).is_err());
    }
}
