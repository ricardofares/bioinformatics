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
