pub enum Target {
    Assembly,
    Brainfuck,
    DT,
    Ook,
    Whitespace,
}

pub fn detect_target(option: Option<String>, filename: &String) -> Option<Target> {
    match option {
        Some(ref val) => match val.as_str() {
            "asm" => Some(Target::Assembly),
            "bf" => Some(Target::Brainfuck),
            "dt" => Some(Target::DT),
            "ook" => Some(Target::Ook),
            "ws" => Some(Target::Whitespace),
            _ => None,
        },
        None => {
            let comps: Vec<&str> = filename.split('.').collect();
            if comps.len() < 2 {
                Some(Target::Whitespace)
            } else {
                match *comps.last().unwrap() {
                    "asm" => Some(Target::Assembly),
                    "bf" => Some(Target::Brainfuck),
                    "dt" => Some(Target::DT),
                    "ook" => Some(Target::Ook),
                    _ => Some(Target::Whitespace),
                }
            }
        }
    }
}
