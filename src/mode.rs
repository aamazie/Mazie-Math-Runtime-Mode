#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Div0Policy {
    Identity,
    Error,
    Ieee,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NonFinitePolicy {
    Allow,
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MazieMode {
    pub div0: Div0Policy,
    pub non_finite: NonFinitePolicy,
}

impl Default for MazieMode {
    fn default() -> Self {
        Self {
            div0: Div0Policy::Identity,
            non_finite: NonFinitePolicy::Allow,
        }
    }
}
