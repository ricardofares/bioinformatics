/*
 * MIT License
 *
 * Copyright (c) 2023 Ricardo Fares
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use bioinformatics::sequencing::global;
use bioinformatics::sequencing::lcs::{lcs, print_lcs};
use bioinformatics::sequencing::local;
use std::env;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut u_seq = String::from("");
    let mut v_seq = String::from("");
    let mut option = String::from("");

    // It checks if the user have not specified arguments
    // when the program has started, then the sequences will
    // be requested from the standard input.
    if args.len() == 1 {
        print!("Insert the first sequence: ");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut u_seq)
            .expect("An error has ocurred in inserting the first sequence");

        // It removes the newline added.
        u_seq.pop();

        print!("Insert the second sequence: ");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut v_seq)
            .expect("An error has ocurred in inserting the second sequence");

        // It removes the newline added.
        v_seq.pop();
    } else {
        // The sequences have been populated from the command-line arguments.
        u_seq = String::from(&args[1]);
        v_seq = String::from(&args[2]);
    }

    println!("\nWhich algorithm would you like to apply?: \n");
    println!(" 1. Longest Common Subsequence.");
    println!(" 2. Needleman-Wunsch (Global Sequence Aligment).");
    println!(" 3. Smith-Waterman (Local Sequence Alignment).");
    println!(" 4. Exit.");
    print!("\nOption: ");

    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut option)
        .expect("An error has ocurred in inserting the user option");

    // It removes the trailing newline.
    option.pop();

    match option.as_str() {
        "1" => {
            // It calculates the longest common subsequence.
            let (_s, b) = lcs(u_seq.as_str(), v_seq.as_str());

            // It terminates printing the longest common subsequence to the user.
            println!("\nThe longest common subsequence is: ");
            print_lcs(b, u_seq.as_str());
        }
        "2" | "3" => {
            let mut match_str = String::from("");
            let mut mismatch_str = String::from("");
            let mut gap_str = String::from("");

            print!("Insert the match point: ");
            let _ = std::io::stdout().flush();
            std::io::stdin()
                .read_line(&mut match_str)
                .expect("An error has occurred when inserting the match point");

            print!("Insert the mismatch point: ");
            let _ = std::io::stdout().flush();
            std::io::stdin()
                .read_line(&mut mismatch_str)
                .expect("An error has occurred when inserting the mismatch point");

            print!("Insert the gap point: ");
            let _ = std::io::stdout().flush();
            std::io::stdin()
                .read_line(&mut gap_str)
                .expect("An error has occurred when inserting the gap point");

            let match_ = match_str
                .trim()
                .parse()
                .expect("Match point is not an integer");
            let mismatch = mismatch_str
                .trim()
                .parse()
                .expect("Mismatch point is not an integer");
            let gap = gap_str.trim().parse().expect("Gap point is not an integer");

            match option.as_str() {
                "2" => {
                    // It calculates the global alignment between the sequences.
                    let (s, b) = global::align_global(
                        u_seq.as_str(),
                        v_seq.as_str(),
                        &global::Options {
                            match_,
                            mismatch,
                            gap,
                        },
                    );

                    println!(
                        "\nThe global sequence alignment with maximum score {} is: ",
                        s[s.row() - 1][s.col() - 1]
                    );
                    global::print_align_global(&b, &u_seq, &v_seq);
                }
                "3" => {
                    // It calculates the global alignment between the sequences.
                    let (s, b) = local::align_local(
                        u_seq.as_str(),
                        v_seq.as_str(),
                        &local::Options {
                            match_,
                            mismatch,
                            gap,
                        },
                    );

                    println!(
                        "\nThe local sequence alignment with maximum score {} is: ",
                        s.max()
                    );
                    local::print_align_local(&s, &b, &u_seq, &v_seq);
                }
                _ => {
                    eprintln!("Unreachable type. It must be `2` or `3`.");
                    std::process::exit(1);
                }
            }
        }
        _ => {
            std::process::exit(0);
        }
    }
}
