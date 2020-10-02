mod sqlstate;
mod type_gen;

fn main() -> std::io::Result<()> {
    type_gen::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/types/gen.rs"))?;
    sqlstate::build(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/state/gen.rs"))
}
