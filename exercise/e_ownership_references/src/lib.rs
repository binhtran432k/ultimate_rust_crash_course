pub fn inspect(arg: &String) {
    let plu_or_sin = if arg.ends_with('s') {
        "plural"
    } else {
        "singular"
    };
    println!("'{}' is {}", arg, plu_or_sin);
}

pub fn change(arg: &mut String) {
    if !arg.ends_with('s') {
        arg.push('s');
    }
}

pub fn eat(arg: String) -> bool {
    arg.starts_with('b') && arg.contains('a')
}

pub fn bedazzle(material: &mut String) {
    *material = "sparkly".to_string();
}
