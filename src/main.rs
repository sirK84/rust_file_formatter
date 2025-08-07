use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

enum Size {
    B(u64),
    KB(u64),
    MB(u64),
    GB(u64),
}

impl Size {
    fn from_string(inp: String) -> Size {
        let mut words = inp.split_whitespace();
        let bytes = words.next().unwrap().parse::<u64>().unwrap();
        let suffix = words.next().unwrap().to_lowercase();
        match suffix.as_str() {
            "b" => Size::B(bytes),
            "kb" => Size::KB(bytes),
            "mb" => Size::MB(bytes),
            "gb" => Size::GB(bytes),
            _ => panic!("Invalid size format: {}", inp),
        }
    }
}

fn format_size(siz: Size) -> Sizes {
    match siz {
        Size::B(bytes) => Sizes{bytes: format!("{} bytes", bytes), kilobytes: format!("{} kilobytes", bytes / 1000), megabytes: format!("{} megabytes", bytes / 1000000), gigabytes: format!("{} gigabytes", bytes / 1000000000)},
        Size::KB(kbytes) => Sizes{bytes: format!("{} bytes", kbytes * 1000), kilobytes: format!("{} kilobytes", kbytes), megabytes: format!("{} megabytes", kbytes / 1000), gigabytes: format!("{} gigabytes", kbytes / 1000000)},
        Size::MB(mbytes) => Sizes{bytes: format!("{} bytes", mbytes * 1000000), kilobytes: format!("{} kilobytes", mbytes * 1000), megabytes: format!("{} megabytes", mbytes), gigabytes: format!("{} gigabytes", mbytes / 1000)},
        Size::GB(gbytes) => Sizes{bytes: format!("{} bytes", gbytes * 1000000000), kilobytes: format!("{} kilobytes", gbytes * 1000000), megabytes: format!("{} megabytes", gbytes * 1000), gigabytes: format!("{} gigabytes", gbytes)},
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No argument provided. Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    // The first argument is the size that was used to call the program. Must use quotes to
    // read this as a single argument
    println!("{:?}", format_size(Size::from_string(args[1].to_string())));
}