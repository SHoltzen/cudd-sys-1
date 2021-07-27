// build.rs
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cudd = Path::new("cudd/");

    env::set_current_dir(cudd);

    let mut configure = println!("{:?}", Command::new(format!("ls")).output().unwrap());
    //let mut configure = Command::new(format!("./configure --enable-shared --prefix={}", out_dir)).status().unwrap();
    let mut configure = Command::new("./configure").arg("--enable-shared").arg("--prefix").arg(out_dir.clone()).status().unwrap();
    let mut make = Command::new("make").status().unwrap();
    let mut make = Command::new("make").arg("install").status().unwrap();

    // Command::new("make").current_dir(&untarred_path);

    // Move all libs to output directory so we don't have to specify each directory
    // run_command( Command::new("sh").current_dir(&out_dir).args(&["-c", "cp cudd-*/*/*.a ."]) ).unwrap();

    println!("cargo:rustc-flags=-L {}/lib", out_dir);
    println!("cargo:rustc-flags=-l static=cudd");
}
