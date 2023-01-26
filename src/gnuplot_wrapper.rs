use anyhow::{Error, Result};
use std::io::{BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

/// gnuplot object
pub struct Gnuplot {
    /// gnuplot process
    process: Child,
    /// gnuplot stdin
    stdin: BufWriter<ChildStdin>,
}

impl Gnuplot {
    pub fn show(config: &str, labels: Vec<&str>, data: Vec<Vec<i32>>) -> Result<()> {

        // generate config string
        let mut config_string = config.to_string();
        config_string.push_str("\nplot '-' using 2:xtic(1) with histogram,\n");
        for _ in data.iter().skip(1) {
            config_string.push_str(&format!("'-' using 2 with histogram, "));
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
        println!("{}", config_string);
        println!("{}", data_string);

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
    pub fn show_example() -> Result<()> {
        // configure gnuplot to plot stacked histogram with labels in first column
        let config = "
            set style data histograms
            set style histogram rowstacked
            set style fill solid border -1
            set boxwidth 0.9 relative
            set xtics rotate by -45 scale 0
            set grid ytics";
        // generate example data with first column as labels and 3 more columns as values
        let data = vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            vec![3, 7, 2, 8, 4, 9, 5, 10, 6, 11],
        ];

        // generate labels for each row
        let labels = vec![
            "label1", "label2", "label3", "label4", "label5", "label6", "label7", "label8", "label9",
            "label10",
        ];
           
        Self::show(config, labels, data)
    }
}
