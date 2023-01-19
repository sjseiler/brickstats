#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Label (String);

impl Label {
    pub fn new(label: &str) -> Label {
        Label(label.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Data {
    value: f64,
    label: Option<Label>,
}

impl Data {
    #[allow(dead_code)]
    pub fn new(value: f64) -> Data {
        Data {
            value,
            label: None,
        }
    }

    pub fn new_with_label(value: f64, label: &str) -> Data {
        Data {
            value,
            label: Some(Label::new(label)),
        }
    }
}

#[allow(dead_code)]
pub struct Bins {
    bin_width: f64,
    bin_count: usize,
    bins: Vec<Label>,
}

pub struct Histogram {
    data: Vec<Data>,
    bins: Bins,
}

/// bins of a histogram
/// bins can be created from data values or labels
impl Bins {
    /// create bins from data values
    /// bin_count: number of bins
    /// bin labels will be generated automatically
    #[allow(dead_code)]
    pub fn new_from_values(data: &Vec<Data>, bin_count: usize) -> Bins {
        let mut min = data[0].value;
        let mut max = data[0].value;
        for d in data {
            if d.value < min {
                min = d.value;
            }
            if d.value > max {
                max = d.value;
            }
        }
        let bin_width = (max - min) / bin_count as f64;
        let mut bins = Vec::new();
        for i in 0..bin_count {
            let bin_min = min + i as f64 * bin_width;
            let bin_max = min + (i + 1) as f64 * bin_width;
            let bin_label = format!("{} - {}", bin_min, bin_max);
            bins.push(Label(bin_label));
        }
        Bins {
            bin_width,
            bin_count,
            bins,
        }
    }

    /// create bins from data labels
    /// bin_count: number of bins
    /// labels will be used as bins
    pub fn new_from_labels(data: &Vec<Data>) -> Bins {
        let mut bins = Vec::new();
        for d in data {
            if let Some(label) = &d.label {
                if !bins.contains(label) {
                    bins.push(label.clone());
                }
            }
        }
        Bins {
            bin_width: 1.0,
            bin_count: bins.len(),
            bins,
        }
    }
}

impl Histogram {

    // create histogram from data and bins
    pub fn new(data: &Vec<Data>, bins: Bins) -> Self {
        let histogram = Histogram {
            data: data.clone(),
            bins: bins,
        };
        histogram
    }

    // plot histogram
    // align labels to the left and bars to the right
    #[allow(dead_code)]
    pub fn plot(&self) {
        let mut counts = vec![0; self.bins.bin_count];
        for d in &self.data {
            let bin_index = d.value as usize;
            counts[bin_index] += 1;
        }
        let max_count = counts.iter().max().unwrap();
        let max_count_len = max_count.to_string().len();
        let max_label_len = self.bins.bins.iter().map(|l| l.0.len()).max().unwrap();
        let max_bar_len = 80 - max_count_len - max_label_len - 4;
        for i in 0..self.bins.bin_count {
            let count = counts[i];
            let bar_len = (count as f64 / *max_count as f64 * max_bar_len as f64) as usize;
            let bar = "=".repeat(bar_len);
            println!("{:width$} | {:>width2$} | {}", self.bins.bins[i].0, count, bar, width = max_label_len, width2 = max_count_len);
        }
    }   

    // plot weighted histogram (data values are weights and added up for each bin)
    // align labels to the left and bars to the right
    pub fn plot_weighted(&self) {
        let mut weighted_labels = Vec::new();
        for label in &self.bins.bins {
            // sum all values from data where label matches
            let sum = self.data.iter().filter(|d| d.label == Some(label.clone())).map(|d| d.value).sum::<f64>();
            weighted_labels.push((label.clone(), sum.round() as i32));
        }

        // sort weighted labels by label string
        weighted_labels.sort_by(|a, b| a.0.cmp(&b.0));
        // weighted_labels.sort_by(|a, b| b.1.cmp(&a.1));
        
        let max_count = weighted_labels.iter().map(|(_, c)| c).max().unwrap();
        let max_count_len = max_count.to_string().len();
        let max_label_len = weighted_labels.iter().map(|(l, _)| l.0.len()).max().unwrap();
        let max_bar_len = 80 - max_count_len - max_label_len - 4;
        for (label, count) in &weighted_labels {
            let bar_len = (*count as f64 / *max_count as f64 * max_bar_len as f64) as usize;
            let bar = "=".repeat(bar_len);
            println!("{:width$} | {:>width2$} | {}", label.0, count, bar, width = max_label_len, width2 = max_count_len);
        }         
    }
}
