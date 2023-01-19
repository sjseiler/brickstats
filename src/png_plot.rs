// use gnuplot to plot stacked histogram
// data has value and rgb color
// rgb color is used to color the bars
// data is grouped by label
// labels are used as y-axis labels
// values are used as x-axis values
// values are stacked on top of each other

use gnuplot::{Figure, self};

#[derive(Debug, Clone, PartialEq)]
pub struct GnuplotData {
    value: f64,
    label: Label,
    color: gnuplot::Color,
}

impl GnuplotData {
    pub fn new(value: f64, label: &str, color: gnuplot::Color) -> GnuplotData {
        GnuplotData {
            value,
            label: Label(label.to_string()),
            color,
        }
    }
}
