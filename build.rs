use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
};

fn main() {
    let exports: Vec<_> = fs::read_dir("./bindings")
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .file_stem()
                .and_then(OsStr::to_str)
                .map(str::to_owned)
        })
        .filter(|f| f != "index")
        .map(|f| format!("export * from \"./{}\"", f))
        .collect();

    let mut file = File::create("./bindings/index.ts").unwrap();
    file.write_all(exports.join("\n").as_bytes()).unwrap();
}
