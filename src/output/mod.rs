mod plot;

use plot::Gnuplot;

pub struct Dataset {
    set_num: String,
    labels: Vec<String>,
    data: Vec<Vec<i32>>,
    color_rgbs: Vec<String>,
}

impl Dataset {
    pub fn new(
        set_num: String,
        labels: Vec<String>,
        data: Vec<Vec<i32>>,
        color_rgbs: Vec<String>,
    ) -> Dataset {
        Dataset {
            set_num,
            labels,
            data,
            color_rgbs,
        }
    }
}

pub fn open_histogram_in_gnuplot(dataset: Dataset) {
    Gnuplot::show(
        &format!(
            "{}\nset title \"Parts of Set {} (including spares)",
            plot::DEFAULT_CONFIG,
            dataset.set_num,
        ),
        dataset.labels,
        dataset.data,
        dataset.color_rgbs,
    )
    .unwrap();
}
