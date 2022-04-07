fn main() {
    doit();
}

fn doit() {
    let t = term::stdout();
    if let Some(mut term) = t {
        eprintln!("stdout found");
        term.fg(term::color::GREEN).unwrap();
        write!(term, "writing this to stdout terminal!\n").unwrap();
        term.reset().unwrap();
        term.flush().unwrap();
    } else {
        eprintln!("no stdout found");
    }
}

#[cfg(test)]
#[test]
fn test_doit() {
    doit();
}
