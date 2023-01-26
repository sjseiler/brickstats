// use gnuplot to plot stacked histogram
// data has value and rgb color
// rgb color is used to color the bars
// data is grouped by label
// labels are used as y-axis labels
// values are used as x-axis values
// values are stacked on top of each other

// helpful links
// https://docs.rs/gnuplot/0.0.29/gnuplot/#enums
// https://gnuplot.sourceforge.net/demo/
// https://gnuplot.sourceforge.net/demo/histograms.html
// https://gnuplot.sourceforge.net/demo/rgb_variable.html

use gnuplot::{AutoOption, AxesCommon, Caption, Color, Figure, PlotOption, TickOption};

// plot a simple histogram with example data
pub fn example_histogram() {
    let tick_options: TickOption<&str> = TickOption::Mirror(false);
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Part Category and Color Distribution", &[])
        .set_x_label("Value", &[])
        .set_y_label("Count", &[])
        .set_x_ticks(Some((AutoOption::Fix(0.1), 1)), &[tick_options], &[])
        .set_y_ticks(Some((AutoOption::Fix(0.1), 1)), &[tick_options], &[])
        .histogram(
            &[
                0.0 as f64, 1.0 as f64, 2.0 as f64, 3.0 as f64, 4.0 as f64, 5.0 as f64, 6.0 as f64,
                7.0 as f64, 8.0 as f64, 9.0 as f64,
            ],
            &[Color("red")],
        );
    fg.echo(&mut std::io::stdout());
    fg.show().unwrap();
}

// plot an example double histogram
pub fn example_double_histogram() {
    let tick_options: TickOption<&str> = TickOption::Mirror(false);
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Part Category and Color Distribution", &[])
        .set_x_label("Value", &[])
        .set_y_label("Count", &[])
        .set_x_ticks(Some((AutoOption::Fix(0.1), 1)), &[tick_options], &[])
        .set_y_ticks(Some((AutoOption::Fix(0.1), 1)), &[tick_options], &[])
        .histogram(
            &[
                0.0 as f64, 1.0 as f64, 2.0 as f64, 3.0 as f64, 4.0 as f64, 5.0 as f64, 6.0 as f64,
                7.0 as f64, 8.0 as f64, 9.0 as f64,
            ],
            &[
                Color("red"),
                Color("blue"),
                Color("green"),
                Color("yellow"),
                Color("orange"),
                Color("purple"),
                Color("black"),
                Color("white"),
            ],
        )
        .histogram(
            &[
                0.0 as f64, 1.0 as f64, 2.0 as f64, 3.0 as f64, 4.0 as f64, 5.0 as f64, 6.0 as f64,
                7.0 as f64, 8.0 as f64, 9.0 as f64,
            ], // range of values
            &[Color("green")],
        );
    fg.echo(&mut std::io::stdout());
    fg.show().unwrap();
}

// plot an example stacked histogram
pub fn example_stacked_histogram() {
    let tick_options: TickOption<&str> = TickOption::Mirror(false);
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Part Category and Color Distribution", &[])
        .set_x_label("Value", &[])
        .set_y_label("Count", &[])
        .set_x_ticks(Some((AutoOption::Fix(1.0), 1)), &[tick_options], &[])
        .set_y_ticks(Some((AutoOption::Fix(10.0), 10)), &[tick_options], &[])
        .set_cb_label("test", &[])
        .set_style("histogram rowstacked")
        .set_style("fill solid 1.0 border lc rgb '#000000'")
        .set_style("data histogram")
        .add_custom_line("set boxwidth 0.8")
        .histogram(
            &[
                0.0 as f64, 1.0 as f64, 2.0 as f64, 3.0 as f64, 4.0 as f64, 5.0 as f64, 6.0 as f64,
                7.0 as f64, 8.0 as f64, 9.0 as f64,
            ],
            &[Color("#000000")],
        )
        .histogram(
            &[
                9.0 as f64, 8.0 as f64, 7.0 as f64, 6.0 as f64, 5.0 as f64, 4.0 as f64, 3.0 as f64,
                2.0 as f64, 1.0 as f64, 0.0 as f64,
            ],
            &[Color("blue")],
        );
    fg.echo(&mut std::io::stdout());
    fg.show().unwrap();
}

// plot an example stacked histogram with labeled bins
pub fn example_stacked_histogram_labeled_bins() {
    let tick_options: TickOption<&str> = TickOption::Mirror(false);
    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Part Category and Color Distribution", &[])
        .set_x_label("Value", &[])
        .set_y_label("Count", &[])
        .set_x_ticks(Some((AutoOption::Fix(1.0), 1)), &[tick_options], &[])
        .set_y_ticks(Some((AutoOption::Fix(10.0), 10)), &[tick_options], &[])
        .set_cb_label("test", &[])
        .set_style("histogram rowstacked")
        .set_style("fill solid 1.0 border lc rgb '#000000'")
        .histogram_labeled_bins(
            &[1.0 as f64, 2.0 as f64, 3.0 as f64, 4.0 as f64],
            &[
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
            ],
            &[Color("#000000")],
        );
    fg.echo(&mut std::io::stdout());
    fg.show().unwrap();
}

// example graph
pub fn example() {
    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();
    // fg.set_terminal("pngcairo" , "plot.png");
    fg.axes2d()
        .lines(&x, &y, &[Caption("A line"), Color("black")]);
    // echo the command to stdout, argument is stdout writer
    fg.echo(&mut std::io::stdout());
    fg.show().unwrap();
}
