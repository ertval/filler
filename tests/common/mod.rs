use std::io::Cursor;

/// Create a simulated stdin from a multiline string
pub fn mock_stdin(input: &str) -> impl std::io::BufRead {
    std::io::BufReader::new(Cursor::new(input.as_bytes().to_vec()))
}

/// Standard 5x5 test anfield as a string block
#[allow(dead_code)]
pub fn fixture_anfield_5x5() -> &'static str {
    "Anfield 5 5:\n    01234\n000 .@...\n001 .....\n\
     002 .....\n003 ...$.\n004 .....\n"
}
