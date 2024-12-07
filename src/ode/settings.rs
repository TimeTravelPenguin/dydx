use std::collections::HashMap;

use anyhow::Result;
use peroxide::fuga::*;
use symbolica::{
    atom::{Atom, Symbol},
    evaluate::{ExpressionEvaluator, FunctionMap, OptimizationSettings},
    symb,
};

use super::schemes::{EmbeddedMethod, OdeSolver};

#[derive(Debug, Clone)]
pub struct OdeSettings {
    pub integration_length: f64,
    pub ode_solver: OdeSolver,
    pub ics: Vec<f64>,
    pub coordinate: OdeCoordinate,
    pub dimensions: u8,
    pub inputs: OdeInputs,
    pub(crate) symbols: HashMap<String, Symbol>,
}

impl Default for OdeSettings {
    fn default() -> Self {
        let mut symbols = HashMap::new();

        for s in &["x", "y", "r", "theta"] {
            symbols.insert(s.to_string(), symb!(s));
        }

        let expr = "x^2 - 7y - 10";

        Self {
            integration_length: 10.0,
            ode_solver: OdeSolver::Embedded(EmbeddedMethod::RKF45),
            ics: vec![1.0, 1.0],
            coordinate: OdeCoordinate::Cartesian,
            dimensions: 1,
            inputs: OdeInputs {
                inputs: vec![expr.to_string()],
                parsed_expressions: Ok(vec![Atom::parse(expr).unwrap()]),
            },
            symbols,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OdeCoordinate {
    Cartesian,
    Polar,
}

#[derive(Debug, Clone)]
pub struct OdeInputs {
    pub inputs: Vec<String>,
    pub parsed_expressions: Result<Vec<Atom>, String>,
}

impl OdeInputs {
    pub fn parse_expressions(&mut self) {
        self.parsed_expressions = self
            .inputs
            .iter()
            .map(|input| Atom::parse(input).map_err(|e| e.to_string()))
            .collect::<Result<Vec<Atom>, String>>();
    }
}
