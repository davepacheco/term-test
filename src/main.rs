use std::io::Write;

fn main() {
    doit();
}

fn doit() {
    eprintln!("println to stderr (stdout found)");
    println!("println to stdout (before)");
    write!(std::io::stdout(), "write to stdout\n").unwrap();
    println!("println to stdout (after)");
}

#[cfg(test)]
#[test]
fn test_doit() {
    doit();
}
