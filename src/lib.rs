use std::io;

use anyhow::Context;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) {
    if pattern.is_empty() {
        panic!("no pattern provided");
    };

    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
                .with_context(|| "results failed to write")
                .unwrap();
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
