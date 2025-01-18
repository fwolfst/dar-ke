use rusty_ppm::*;
use std::{env, path::Path};

/// Create simple bitmap from ppm
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Need an argument (a ppm file)");
    }

    let img = ppm_reader::read_ppm(Path::new("./"), args[1].as_str()).unwrap();

    let data = img.data;
    // TODO find a better, emptylooking identifier
    println!("const ˑ: bool = true;");
    println!("const Ø: bool = false;");
    println!("const BITMAP: [[bool; {}]; {}] = [", img.width, img.height);
    for y in 0..img.height {
        print!("  [");
        for x in 0..img.width {
            if data[y * img.width + x].z > 0 {
                print!("ˑ,");
            } else {
                print!("Ø,");
            }
        }
        println!("],");
    }
    println!("];");
}
