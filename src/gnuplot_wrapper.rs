use anyhow::{Error, Result};
use std::io::{BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

pub const DEFAULT_CONFIG: &str = "
set palette rgbformulae 3,2,2
set palette maxcolors 256
unset colorbox
set style data histograms
set style histogram rowstacked
set style fill solid border -1
set boxwidth 0.9 relative
set xtics rotate by -45 scale 0
set grid ytics";

/// gnuplot object
pub struct Gnuplot {
    // gnuplot process
    // process: Child,
    // gnuplot stdin
    // stdin: BufWriter<ChildStdin>,
}

impl Gnuplot {
    pub fn show(config: &str, labels: Vec<String>, data: Vec<Vec<i32>>, colors: Vec<String>) -> Result<()> {
        let mut color_iter = colors.iter();
        // generate config string
        let mut config_string = config.to_string();
        config_string.push_str(&format!("\nplot '-' using 2:xtic(1) with histogram notitle lc rgb \"#{}\",", color_iter.next().expect("didn't get enough colors")));
        for _ in data.iter().skip(1) {
            config_string.push_str(&format!(" '-' using 2 with histogram notitle lc rgb \"#{}\",", color_iter.next().expect("didn't get enough colors")));
        }
        // remove last comma
        config_string.pop();

        let mut data_string = String::new();
        // generate data string from labels and data
        // make iterator from data
        for d in data.iter() {
            let mut data = labels.iter()
                .zip(d.iter())
                .map(|(label, value)| format!("\"{}\" {}", label, value))
                .collect::<Vec<String>>()
                .join("\n");
            // add end of data marker
            data.push_str("\ne\n");
            data_string.push_str(&data);
        }   

        // print config and data
        // println!("{}", config_string);
        // println!("{}", data_string);

        let mut process = Command::new("gnuplot")
            .arg("-p")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH.");
        let mut stdin = BufWriter::new(process.stdin.take().unwrap());
        writeln!(stdin, "{}", config_string)?;
        writeln!(stdin, "{}", data_string)?;
        stdin.flush()?;

        match process.wait() {
            Ok(status) => {
                if status.success() {
                    Ok(())
                } else {
                    Err(Error::msg("gnuplot exited with non-zero status"))
                }
            }
            Err(e) => Err(Error::msg(format!("gnuplot failed to run: {}", e))),
        }
    }
    // pub fn show_example() -> Result<()> {
    //     // configure gnuplot to plot stacked histogram with labels in first column
    //     let config = "
    //         set style data histograms
    //         set style histogram rowstacked
    //         set style fill solid border -1
    //         set boxwidth 0.9 relative
    //         set xtics rotate by -45 scale 0
    //         set grid ytics";
    //     // generate example data with first column as labels and 3 more columns as values
    //     let data = vec![
    //         vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    //         vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    //         vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
    //         vec![3, 7, 2, 8, 4, 9, 5, 10, 6, 11],
    //     ];

    //     // generate example colors for each of the 10 column
    //     let colors = vec![
    //         "FFFFFF", "253822", "a8978e", "45eadf", "123456", "000000", "acd234", "aef238", "534dd3",
    //         "000000",
    //     ];


    //     // generate labels for each row
    //     let labels = vec![
    //         "label1", "label2", "label3", "label4", "label5", "label6", "label7", "label8", "label9",
    //         "label10",
    //     ];
           
    //     Self::show(config, labels, data, colors)
    // }
}
