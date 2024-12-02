use anyhow::{anyhow, Context, Result};
use args::Cli;
use clap::Parser;
use dotenv_codegen::dotenv;
use lazy_static::lazy_static;
use log::*;
use nannou::prelude::{map_range, pt2, srgb, App, Draw, Frame, Rect, Update, BLACK, RED};
use nannou_egui::{
    egui::{self, RichText, TextStyle},
    Egui,
};
use ode::{solve_ode, OdeCoordinate, OdeSettings};
use std::env::{set_var, var};
use std::panic;
use symbolica::{atom::Atom, printer::PrintOptions, LicenseManager};

mod args;
mod fonts;
mod ode;

lazy_static! {
    pub static ref CLI: Cli = Cli::parse();
}

fn main() -> Result<()> {
    LicenseManager::set_license_key(dotenv!("SYMBOLICA_LICENSE"))
        .map_err(|e| anyhow!("Failed to set license key: {}", e))?;

    let log_level = CLI.verbose.log_level().context("Failed to get log level")?;
    init_logging(log_level);

    panic::set_hook(Box::new(|panic_info| {
        eprintln!("Application panicked: {}", panic_info);
    }));

    nannou::app(model).update(update).run();

    Ok(())
}

fn init_logging(level: Level) {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let mut overridden = false;
    let verbosity = if std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| "0".into()) == "1" {
        overridden = true;
        "trace"
    } else {
        match level {
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
            Level::Debug => "debug",
            _ => "trace",
        }
    };

    let crate_name = env!("CARGO_PKG_NAME");
    set_var("RUST_LOG", format!("{}={}", crate_name, verbosity));

    pretty_env_logger::init();

    if overridden {
        warn!("RUST_BACKTRACE is set, overriding user verbosity level");
    } else if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
        trace!("RUST_BACKTRACE has been set");
    };
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").expect("Should set RUST_LOG environment variable")
    );
}

struct PlotSettings {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
}

impl Default for PlotSettings {
    fn default() -> Self {
        Self {
            x_min: -10.0,
            x_max: 10.0,
            y_min: -10.0,
            y_max: 10.0,
        }
    }
}

struct Settings {
    ode_settings: ode::OdeSettings,
    plot_settings: PlotSettings,
}

struct Model {
    settings: Settings,
    egui: Egui,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();
    let mut egui = Egui::from_window(&window);

    info!("Setting fonts");
    fonts::set_fonts(&mut egui);

    Model {
        egui,
        settings: Settings {
            ode_settings: OdeSettings::default(),
            plot_settings: PlotSettings::default(),
        },
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    update_egui(model, update);

    let egui_wants_pointer = model.egui.ctx().wants_pointer_input();

    // Update model only if the left mouse button is down and egui doesn't want the pointer input.
    if !egui_wants_pointer && app.mouse.buttons.left().is_down() {
        // TODO: Ensure 2D
        let (x, y) = screen_to_point(
            &model.settings.plot_settings,
            &app.window_rect(),
            app.mouse.x.into(),
            app.mouse.y.into(),
        );
        debug!("Mouse left: ({}, {})", x, y);
        model.settings.ode_settings.ics = vec![x, y];
    }

    // change x/y bound on scroll
}

fn update_egui(model: &mut Model, update: Update) {
    let settings = &mut model.settings;
    let egui = &mut model.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    let ode_settings = &mut settings.ode_settings;
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.radio_value(
                &mut ode_settings.coordinate,
                OdeCoordinate::Cartesian,
                "Cartesian",
            )
            .on_hover_text("Cartesian coordinate system (x, y)");

            ui.radio_value(&mut ode_settings.coordinate, OdeCoordinate::Polar, "Polar")
                .on_hover_text("Polar coordinate system (r, θ)");
        });

        ui.separator();

        if ode_settings.dimensions == 1 {
            let ode_input = &mut ode_settings.inputs;

            ui.label("Input ODE");
            ui.horizontal(|ui| {
                ui.label("f(x, y) =");
                let response = ui.text_edit_singleline(&mut ode_input.inputs[0]);

                if response.changed() {
                    let expr = Atom::parse(&ode_input.inputs[0]);
                    if let Ok(expr) = expr {
                        ode_input.parsed_expressions = Ok(vec![expr]);
                    }
                }
            });
        } else {
            ui.label("ODE is a system -- Coming soon!");
        }

        let opts = PrintOptions {
            terms_on_new_line: false,
            color_top_level_sum: false,
            color_builtin_symbols: false,
            print_finite_field: true,
            symmetric_representation_for_finite_field: false,
            explicit_rational_polynomial: false,
            number_thousands_separator: None,
            multiplication_operator: '*',
            double_star_for_exponentiation: false,
            square_brackets_for_function: false,
            num_exp_as_superscript: true,
            latex: false,
        };

        ode_settings.inputs.inputs.iter().for_each(|input| {
            let value = Atom::parse(input)
                .map(|p| format!("{}", p.printer(opts)))
                .unwrap_or("".to_string());

            let value = value.replace("theta", "θ");
            ui.label(RichText::new(value).text_style(TextStyle::Name("STIXRegular".into())));
        });

        ui.separator();

        ui.label("initial conditions");
        ui.horizontal(|ui| {
            for (i, ic) in ode_settings.ics.iter_mut().enumerate() {
                ui.label(format!("{}:", i));
                ui.add(egui::DragValue::new(ic).speed(0.1));
            }
        });

        ui.label(format!(
            "Integration length: {}",
            ode_settings.integration_length
        ));
        ui.add(
            egui::DragValue::new(&mut ode_settings.integration_length)
                .speed(0.1)
                .clamp_range(0..=20),
        );
    });
}

fn draw_plot(draw: &Draw, win: &Rect, model: &Model, domain: &[f64], image: &[f64]) -> Result<()> {
    let settings = &model.settings;
    let plot_settings = &settings.plot_settings;
    let ode_settings = &settings.ode_settings;

    let col = srgb(31.0 / 255.0, 101.0 / 255.0, 245.0 / 255.0);

    let vertices = domain.iter().zip(image).map(|(&x, &y)| {
        let (mut x, mut y) = (x, y);

        if ode_settings.coordinate == OdeCoordinate::Polar {
            let r = x;
            let theta = y;
            x = r * theta.cos();
            y = r * theta.sin();
        }

        let (x, y) = point_to_screen(plot_settings, win, x, y);
        (pt2(x as f32, y as f32), col)
    });

    let vertices = vertices.clone().take_while(|(p, _)| {
        let is_finite = p.x.is_finite() && p.y.is_finite();

        if !is_finite {
            let x_pt = map_range(
                p.x,
                win.left(),
                win.right(),
                plot_settings.x_min,
                plot_settings.x_max,
            );
            warn!("Found non-finite point: {:?} (at x = {:?})", p, x_pt);
        }

        is_finite
    });

    // Draw the polyline as a stroked path.
    let weight = 2.0;
    draw.polyline()
        .weight(weight)
        .join_round()
        .points_colored(vertices);

    Ok(())
}

fn compute_ode_soln(ode_settings: &OdeSettings) -> Result<(Vec<f64>, Vec<Vec<f64>>)> {
    if ode_settings.dimensions == 1 {
        // TODO: Make animated 2D wavey bois
        let (mut x0, mut y0) = (ode_settings.ics[0], ode_settings.ics[1]);
        let mut xn = x0 + ode_settings.integration_length;

        if ode_settings.coordinate == OdeCoordinate::Polar {
            let x = x0;
            let y = y0;
            x0 = (x.powi(2) + y.powi(2)).sqrt();
            xn = (xn.powi(2) + y.powi(2)).sqrt();
            y0 = f64::atan2(y, x);
        }

        solve_ode(
            ode_settings,
            (x0, xn),
            1e-3,
            //&ode_settings.ics,
            &[y0],
        )
    } else {
        anyhow::bail!("System ODEs not implemented yet");
    }
}

fn point_to_screen(plot_settings: &PlotSettings, win: &Rect, x: f64, y: f64) -> (f64, f64) {
    let x = map_range(
        x,
        plot_settings.x_min,
        plot_settings.x_max,
        win.left().into(),
        win.right().into(),
    );
    let y = map_range(
        y,
        plot_settings.y_min,
        plot_settings.y_max,
        win.bottom().into(),
        win.top().into(),
    );
    (x, y)
}

fn screen_to_point(plot_settings: &PlotSettings, win: &Rect, x: f64, y: f64) -> (f64, f64) {
    let x = map_range(
        x,
        win.left().into(),
        win.right().into(),
        plot_settings.x_min,
        plot_settings.x_max,
    );
    let y = map_range(
        y,
        win.bottom().into(),
        win.top().into(),
        plot_settings.y_min,
        plot_settings.y_max,
    );
    (x, y)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    let settings = &model.settings;

    draw.background().color(BLACK);

    let ode_soln = compute_ode_soln(&settings.ode_settings);

    match ode_soln {
        Ok((domain, image)) => {
            let image = image.into_iter().flatten().collect::<Vec<_>>();
            draw_plot(&draw, &win, model, &domain, image.as_slice())
                .unwrap_or_else(|e| error!("Error drawing plot: {}", e));
        }
        Err(e) => {
            error!("Failed to solve ODE: {}", e);
        }
    }

    draw_ic(&draw, &win, &settings.ode_settings);

    draw.to_frame(app, &frame)
        .unwrap_or_else(|e| error!("Error drawing frame: {:?}", e));

    model
        .egui
        .draw_to_frame(&frame)
        .unwrap_or_else(|e| error!("Error drawing egui: {}", e));
}

fn draw_ic(draw: &Draw, win: &Rect, ode_settings: &OdeSettings) {
    let (x0, y0) = (ode_settings.ics[0], ode_settings.ics[1]);
    let (x, y) = point_to_screen(&PlotSettings::default(), win, x0, y0);

    draw.ellipse()
        .x_y(x as f32, y as f32)
        .radius(5.0)
        .color(RED);
}
