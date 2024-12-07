#![allow(dead_code)]

use std::collections::HashMap;

use anyhow::Result;
use peroxide::fuga::*;
use symbolica::{
    atom::{Atom, Symbol},
    evaluate::{ExpressionEvaluator, FunctionMap, OptimizationSettings},
    symb,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ExplicitMethod {
    /// Ralston's 3rd order method
    RALS3,
    /// Runge-Kutta 4th order method
    RK4,
    /// Ralston's 4th order method
    RALS4,
    /// Runge-Kutta 5th order method
    RK5,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImplicitMethod {
    /// Gauss-Legendre 4th order method
    GL4,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EmbeddedMethod {
    /// Bogacki-Shampine 2/3rd order method
    BS23,
    /// Runge-Kutta-Fehlberg 4/5th method
    RKF45,
    /// Dormand-Prince 4/5th order method
    DP45,
    /// Tsitouras 4/5th order method
    TSIT45,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OdeSolver {
    Explicit(ExplicitMethod),
    Implicit(ImplicitMethod),
    Embedded(EmbeddedMethod),
}
