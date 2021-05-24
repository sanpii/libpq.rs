fn main() {
    use std::io::Write;

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("version.rs");
    let mut f = std::fs::File::create(&dest_path).unwrap();

    let versions = env!("CARGO_PKG_VERSION").split('.').collect::<Vec<_>>();
    let major = versions[0];
    let minor = versions[1].parse::<i32>().unwrap();
    let pg_version_num = format!("{}{:04}", major, minor);

    write!(&mut f, "const PG_VERSION_NUM: i32 = {};", pg_version_num)
        .unwrap();

    println!("cargo:rerun-if-env-changed=PG_VERSION_NUM");
}
