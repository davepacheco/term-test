fn main() {
    doit();
}

fn doit() {
    let t = term::stdout();
    if let Some(mut term) = t {
        eprintln!("println to stderr (stdout found)");
        println!("println to stdout (before)");
        term.fg(term::color::GREEN).unwrap();
        write!(term, "terminal write to stdout\n").unwrap();
        term.reset().unwrap();
        term.flush().unwrap();
        println!("println to stdout (after)");
    } else {
        eprintln!("println to stderr (stdout not found)");
    }
}

#[cfg(test)]
#[test]
fn test_doit() {
    doit();
}
