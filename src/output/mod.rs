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

    pub fn output(&self, output: Option<String>) {
        Gnuplot::output(
            self.labels.clone(),
            self.data.clone(),
            self.color_rgbs.clone(),
            self.set_num.clone(),
            output,
        )
        .unwrap();
    }
}
