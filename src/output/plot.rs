use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};

// convert to png with something similar to >> set terminal pngcairo enhanced font \"Times New Roman,12.0\" size 1500,1100
pub const DEFAULT_CONFIG: &str = "
set lmargin screen 0.05
set rmargin screen 0.95
set tmargin screen 0.9
set palette rgbformulae 3,2,2
set palette maxcolors 256
unset colorbox
set style data histograms
set style histogram rowstacked
set style fill solid border -1
set boxwidth 0.9 relative
set key autotitle columnheader 
set ytics auto
#set mytics 5
set xtics rotate by -45 scale 0
set grid ytics
set title font \"Helvetica,20\"
";

#[derive(Debug, Deserialize, Serialize)]
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

    pub fn output(&self, output: Option<String>, title: String) {
        Gnuplot::output(
            self.labels.clone(),
            self.data.clone(),
            self.color_rgbs.clone(),
            title,
            output,
        )
        .unwrap();
    }
}

// gnuplot object
pub struct Gnuplot {
    // gnuplot process
    // process: Child,
    // gnuplot stdin
    // stdin: BufWriter<ChildStdin>,
}

impl Gnuplot {
    // create output fn that replaces show and save_png
    pub fn output(
        labels: Vec<String>,
        data: Vec<Vec<i32>>,
        colors: Vec<String>,
        title: String,
        output: Option<String>,
    ) -> Result<()> {
        let (config_string, data_string) =
            Gnuplot::prepare_plotting(labels, data, colors, output, title);

        let mut process = Command::new("gnuplot")
            .arg("-p")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Couldn't spawn gnuplot. Make sure it is installed and available in PATH.");
        {
            let mut stdin = BufWriter::new(process.stdin.take().unwrap());
            writeln!(stdin, "{}", config_string)?;
            writeln!(stdin, "{}", data_string)?;
            stdin.flush()?;
        }

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

    fn prepare_plotting(
        labels: Vec<String>,
        data: Vec<Vec<i32>>,
        colors: Vec<String>,
        file_name_prefix: Option<String>,
        title: String,
    ) -> (String, String) {
        let mut color_iter = colors.iter();
        // generate config string
        let mut config_string = format!("{DEFAULT_CONFIG}\nset title \"{title}\"",);
        if let Some(file_name) = file_name_prefix {
            config_string.push_str(&format!("\nset output '{}_histogram.png'", file_name));
            config_string.push_str(
                "\nset terminal pngcairo enhanced font \"Times New Roman,12.0\" size 1920,1080",
            );
        }

        config_string.push_str(&format!(
            "\nplot '-' using 2:xtic(1) with histogram notitle lc rgb \"#{}\",",
            color_iter.next().expect("didn't get enough colors")
        ));
        for _ in data.iter().skip(1) {
            config_string.push_str(&format!(
                " '-' using 2 with histogram notitle lc rgb \"#{}\",",
                color_iter.next().expect("didn't get enough colors")
            ));
        }
        // remove last comma
        config_string.pop();

        let mut data_string = String::new();
        // generate data string from labels and data
        // make iterator from data
        for d in data.iter() {
            let mut data = labels
                .iter()
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
        (config_string, data_string)
    }
}
