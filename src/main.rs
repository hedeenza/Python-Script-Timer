use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
fn main() {
    // Create a vector to hold each line of the Python script
    let mut input_vector = Vec::new();

    // Load in the script to be timed
    let input_arg = env::args().nth(1);
    // Match the input
    match input_arg {
        // If an argument was provided...
        Some(arg) => {
            // Open the file
            let input_file = File::open(arg).unwrap();
            // Load the file into the BufReader
            let input_reader = BufReader::new(input_file);
            // For each line in the Pyhton script...
            for line in input_reader.lines() {
                match line {
                    // If there is a line...
                    Ok(content) => {
                        // Push it to the input_vector
                        input_vector.push(content);
                    }
                    // If there is not a line...
                    Err(err) => {
                        // Print the error
                        println!("{:?}", err);
                    }
                }
            }
        }
        // If an argument was provided...
        None => println!("No file provided [ Argument 1 ]"),
    }

    // Add the line to import the time module at the beginning of the script
    input_vector.insert(0, "".to_string());
    input_vector.insert(0, "import time".to_string());
    input_vector.insert(0, "".to_string());
    // Add an empty value to the end of the input_vector so the window has room to reach the end
    input_vector.push("".to_string());

    // Set the window size to 2
    let chunks = input_vector.windows(2);

    let mut timers_added = Vec::new();
    let mut timer_results = Vec::new();

    // Start with i negative to skip trying to insert timers around the "import time" line
    let mut i = -2;

    // For each window...
    for index in chunks {
        // If there is a window...
        match Some(index) {
            Some(string) => {
                // If we're still in the "import time section"
                if i < 0 {
                    // Add 1 to i
                    i += 1;
                    let a = string[0].to_string();
                    timers_added.push(a);
                    // Don't do anything else
                    continue;
                } else {
                    // If the first window slow is empty and the second window slot is not...
                    // In other words... if the beginning of a contiguous code block is detected 
                    if index[0].trim() == "" && index[1].trim() != "" {
                        let b = index[0].to_string();
                        timers_added.push(b); // Add the blank line to the timers_added vector
                        let c = format!("time_start_{} = time.time()", i);
                        timers_added.push(c); // Add the start timer to the timers_added vector
                    // If the first window slow is not empty and the second window slot is ...
                    // In other words... if the end of a contiguous code block is detected 
                    } else if index[0].trim() != "" && index[1].trim() == "" {
                        let d = index[0].to_string();
                        timers_added.push(d); // Add the last line of the block to the timers_added vector
                        let e = format!("time_end_{} = time.time()", i);
                        timers_added.push(e); // Add the end timer to the timers_added vector
                        let f = format!("elapsed_{} = time_end_{} - time_start_{}", i, i, i);
                        timers_added.push(f); // Add the elapsed time calculation to the timers_added vector

                        let g = index[0].to_string();
                        let g2 = g.replace(['\"', '\'', '{', '}'], " "); // Replace special
                                                                         // characters that will
                                                                         // interfere with printing
                        let result = format!(
                            "print(f\"Block ending with: \'{}\' | Time = {{round(elapsed_{}, 2)}}\")",
                            g2, i
                        ); // Format the result of the timer for the block
                        timer_results.push(result); // Add the block's timer to the timer_results vector

                        i += 1;
                    // If both window slots are empty or full
                    } else {
                        let h = string[0].to_string();
                        timers_added.push(h); // Add the content of the first slot to the timers_added vector
                    }
                }
            }
            None => println!("No more windows"),
        };
    }

    // Create an output file
    let mut output_file = File::create("with_timers.py").expect("Could not create temporary file");

    // Write all of the script lines to the output file
    for line in timers_added {
        let _ = writeln!(output_file, "{}", line);
    }

    // Write a blank line 
    let _ = writeln!(output_file, " ");

    // Write all the timer results lines to the end of the script
    for timer in timer_results {
        let _ = writeln!(output_file, "{}", timer);
    }

    // Run the Python script that contains all the timers
    let _ = Command::new("python").arg("with_timers.py").spawn();
}
