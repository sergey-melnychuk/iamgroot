use iamgroot::{gen_code, gen_json};

fn main() {
    env_logger::init();

    let (mode, paths) = {
        let mut args = std::env::args().skip(1);
        let mode = args.next().expect("Output: [JSON, TREE, CODE].");
        let paths = args.collect::<Vec<_>>();
        (mode, paths)
    };

    if paths.is_empty() {
        eprintln!("No input files");
        return;
    }

    if mode.as_str() == "JSON" {
        if paths.len() > 1 {
            eprintln!("JSON mode supports only single file");
            return;
        }
        println!("{}", gen_json(&paths[0]));
    } else if mode.as_str() == "CODE" {
        println!("{}", gen_code(&paths).unwrap());
    } else {
        eprintln!("Unknown mode: {mode}. Supported are: JSON, TREE, CODE.");
    }
}
