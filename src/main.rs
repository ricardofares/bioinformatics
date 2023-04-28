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
use bioinformatics::lcs::{lcs, print_lcs};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut u_seq = String::from("");
    let mut v_seq = String::from("");

    // It checks if the user have not specified arguments
    // when the program has started, then the sequences will
    // be requested from the standard input.
    if args.len() == 1 {
        println!("Insert the first sequence: ");
        std::io::stdin()
            .read_line(&mut u_seq)
            .expect("An error has occured in inserting the first sequence");

        println!("\nInsert the second sequence: ");
        std::io::stdin()
            .read_line(&mut v_seq)
            .expect("An error has occured in inserting the second sequence");
    } else {
        // The sequences have been populated from the command-line arguments.
        u_seq = String::from(&args[1]);
        v_seq = String::from(&args[2]);
    }

    // It calculates the longest common subsequence.
    let (_s, b) = lcs(u_seq.as_str(), v_seq.as_str());

    // It terminates printing the longest common subsequence to the user.
    println!("\nThe longest common subsequence is: ");
    print_lcs(b, u_seq.as_str());
}