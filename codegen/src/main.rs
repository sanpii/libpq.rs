mod sqlstate;
mod type_gen;

fn main() -> std::io::Result<()> {
    type_gen::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/ty/gen.rs"))
}

fn snake_to_camel(s: &str) -> String {
    let mut out = String::new();

    let mut upper = true;
    for ch in s.chars() {
        if ch == '_' {
            upper = true;
        } else {
            let ch = if upper {
                upper = false;
                ch.to_ascii_uppercase()
            } else {
                ch
            };
            out.push(ch);
        }
    }

    out
}
