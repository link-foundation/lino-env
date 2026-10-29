use lino_env::read_lino_env;
use std::collections::HashMap;
use std::fs;

#[test]
fn preferences_have_equivalent_line_endings_and_roundtrip() {
    let path = std::env::temp_dir().join(format!("lino-env-endings-{}.lenv", std::process::id()));
    let path = path.to_str().unwrap();
    let expected: HashMap<String, String> = [
        ("LAZADA_MARKET", "vn"),
        ("CITY", "Nha Trang"),
        ("ENABLED", "true"),
        ("DISABLED", "false"),
        ("SORT", "newest"),
        ("FLAVOUR", "chocolate"),
        ("ALTERNATIVE", "unflavoured"),
        ("SPACE", "  preserve  "),
        ("EMPTY", ""),
        ("DOUBLE", "\"quoted value\""),
        ("SINGLE", "'quoted value'"),
        ("LITERAL_CR", "left\rright"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect();

    for ending in ["\n", "\r\n"] {
        for final_newline in [false, true] {
            let mut lines = vec![
                "# Buying preferences".to_string(),
                "LAZADA_MARKET: th".to_string(),
            ];
            lines.extend(
                expected
                    .iter()
                    .map(|(key, value)| format!("{key}: {value}")),
            );
            let mut content = lines.join(ending);
            if final_newline {
                content.push_str(ending);
            }
            fs::write(path, content).unwrap();
            let env = read_lino_env(path).unwrap();
            assert_eq!(env.to_hash_map(), expected);
            env.write().unwrap();
            assert_eq!(read_lino_env(path).unwrap().to_hash_map(), expected);
        }
    }
    fs::remove_file(path).unwrap();
}

#[test]
fn multiline_quotes_and_mixed_endings_preserve_content() {
    let path = std::env::temp_dir().join(format!(
        "lino-env-block-endings-{}.lenv",
        std::process::id()
    ));
    let path = path.to_str().unwrap();
    for ending in ["\n", "\r\n"] {
        for quote in ['\'', '"'] {
            let content = [
                format!("BLOCK: {quote}"),
                "  CITY: Nha Trang  ".to_string(),
                String::new(),
                "  ENABLED: true".to_string(),
                format!("  {quote}"),
                "AFTER: unflavoured".to_string(),
            ]
            .join(ending);
            fs::write(path, content).unwrap();
            let env = read_lino_env(path).unwrap();
            assert_eq!(
                env.get("BLOCK").unwrap(),
                "\n  CITY: Nha Trang  \n\n  ENABLED: true\n  "
            );
            assert_eq!(env.get("AFTER").unwrap(), "unflavoured");
            assert!(!env.has("CITY"));
            assert!(!env.has("ENABLED"));
        }
    }
    fs::write(
        path,
        "A: first\r\nA: last  \nB: left\rright\r\nC: keep\r\r\nD: final\r",
    )
    .unwrap();
    let env = read_lino_env(path).unwrap();
    assert_eq!(env.get("A").unwrap(), "last  ");
    assert_eq!(env.get("B").unwrap(), "left\rright");
    assert_eq!(env.get("C").unwrap(), "keep\r");
    assert_eq!(env.get("D").unwrap(), "final\r");
    fs::remove_file(path).unwrap();
}
