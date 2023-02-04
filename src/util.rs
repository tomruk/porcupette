use std::{fs::OpenOptions, io::Write};

// For debugging purposes
#[allow(dead_code)]
pub fn write_args() {
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open("/tmp/args")
        .unwrap();

    let args = std::env::args();
    let args: Vec<String> = args.collect();

    f.write_all(args.join(" ").as_bytes()).unwrap();
    f.write_all(b"\n\n").unwrap();
}
