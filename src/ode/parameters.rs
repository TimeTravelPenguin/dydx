#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SafetyFactor(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxStepSize(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinStepSize(f64);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct MaxSteps(usize);
