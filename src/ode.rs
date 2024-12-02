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

#[derive(Debug, Clone)]
pub struct OdeSettings {
    pub integration_length: f64,
    pub ics: Vec<f64>,
    pub coordinate: OdeCoordinate,
    pub dimensions: u8,
    pub inputs: OdeInputs,
    symbols: HashMap<String, Symbol>,
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

struct MaxStepODESolver<I: ODEIntegrator> {
    integrator: I,
}

struct ParsedODEProblem {
    dimensions: u8,
    evaluator: ExpressionEvaluator<f64>,
}

impl ParsedODEProblem {
    pub fn create(settings: &OdeSettings) -> Result<Self> {
        let expressions = settings
            .inputs
            .parsed_expressions
            .clone()
            .map_err(|e| anyhow::anyhow!("Failed to parse expressions: {}", e))?;

        let expressions = expressions
            .iter()
            .map(|expr| expr.as_view())
            .collect::<Vec<_>>();

        let symbols = match settings.coordinate {
            OdeCoordinate::Cartesian => &["x", "y"],
            OdeCoordinate::Polar => &["r", "theta"],
        }
        .iter()
        .map(|s| Atom::new_var(settings.symbols[*s]))
        .collect::<Vec<_>>();

        let evaluator = Atom::evaluator_multiple(
            expressions.as_slice(),
            &FunctionMap::new(),
            symbols.as_slice(),
            OptimizationSettings::default(),
        )
        .map_err(|e| anyhow::anyhow!("Failed to create evaluator: {:?}", e))?
        .map_coeff(&|x| x.into());

        Ok(Self {
            dimensions: settings.dimensions,
            evaluator,
        })
    }
}

impl<I: ODEIntegrator> ODESolver for MaxStepODESolver<I> {
    fn solve<P: ODEProblem>(
        &self,
        problem: &P,
        t_span: (f64, f64),
        dt: f64,
        initial_conditions: &[f64],
    ) -> Result<(Vec<f64>, Vec<Vec<f64>>)> {
        let mut t = t_span.0;
        let mut dt = dt;
        let mut y = initial_conditions.to_vec();
        let mut t_vec = vec![t];
        let mut y_vec = vec![y.clone()];

        while t < t_span.1 {
            let dt_step = self.integrator.step(problem, t, &mut y, dt);

            if let Err(e) = &dt_step {
                if let Some(ODEError::ReachedMaxStepIter) = e.downcast_ref() {
                    break;
                }
            }

            let dt_step = dt_step?;

            t += dt;
            t_vec.push(t);
            y_vec.push(y.clone());
            dt = dt_step;
        }

        Ok((t_vec, y_vec))
    }
}

impl ODEProblem for ParsedODEProblem {
    fn rhs(&self, t: f64, y: &[f64], dy: &mut [f64]) -> Result<()> {
        if y.len() != self.dimensions as usize {
            anyhow::bail!(
                "y has the wrong length. Expected {}, got {}",
                self.dimensions,
                y.len()
            );
        }

        if dy.len() != self.dimensions as usize {
            anyhow::bail!(
                "dy has the wrong length. Expected {}, got {}",
                self.dimensions,
                dy.len()
            );
        }

        let in_ = std::iter::once(&t).chain(y).copied().collect::<Vec<_>>();

        let evaluator = &mut self.evaluator.clone();
        evaluator.evaluate(in_.as_slice(), dy);

        Ok(())
    }
}

pub fn solve_ode(
    settings: &OdeSettings,
    t_span: (f64, f64),
    dt: f64,
    ics: &[f64],
) -> Result<(Vec<f64>, Vec<Vec<f64>>)> {
    let solver = ParsedODEProblem::create(settings)?;

    let rkf45 = RKF45::new(1e-4, 0.9, 1e-6, 1e-2, 1000);
    let ode_solver = MaxStepODESolver { integrator: rkf45 };

    ode_solver.solve(&solver, t_span, dt, ics)
}
