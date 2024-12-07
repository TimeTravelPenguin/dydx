#![allow(dead_code)]

use nannou::color::Srgba;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XAxisLocation {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YAxisLocation {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisLocation {
    X(XAxisLocation),
    Y(YAxisLocation),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisLimit {
    Min(f64),
    Max(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridLine {
    Major,
    Minor,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineStyle {
    Solid,
    Dashed(f64),
    Dotted(f64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridConfig {
    visible_lines: GridLine,
    major_line_width: f64,
    minor_line_width: f64,
    major_line_color: Srgba,
    minor_line_color: Srgba,
    major_line_style: LineStyle,
    minor_line_style: LineStyle,
    major_line_interval: f64,
    minor_line_interval: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    x_axis_location: XAxisLocation,
    y_axis_location: YAxisLocation,
}

impl Default for Axes {
    fn default() -> Self {
        Self {
            min_x: -10.0,
            max_x: 10.0,
            min_y: -10.0,
            max_y: 10.0,
            x_axis_location: XAxisLocation::Center,
            y_axis_location: YAxisLocation::Center,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxesBuilder {
    axes: Axes,
}

impl AxesBuilder {
    pub fn new() -> Self {
        Self {
            axes: Axes::default(),
        }
    }

    pub fn set_limits(mut self, axis: Axis, limits: AxisLimit) -> Self {
        match axis {
            Axis::X => match limits {
                AxisLimit::Min(min_x) => self.axes.min_x = min_x,
                AxisLimit::Max(max_x) => self.axes.max_x = max_x,
            },
            Axis::Y => match limits {
                AxisLimit::Min(min_y) => self.axes.min_y = min_y,
                AxisLimit::Max(max_y) => self.axes.max_y = max_y,
            },
        }

        self
    }

    pub fn set_axis_location(mut self, axis_location: AxisLocation) -> Self {
        match axis_location {
            AxisLocation::X(x_axis_location) => self.axes.x_axis_location = x_axis_location,
            AxisLocation::Y(y_axis_location) => self.axes.y_axis_location = y_axis_location,
        }

        self
    }

    pub fn build(self) -> Axes {
        self.axes
    }
}
