fn main() {
    println!("{:?}", raindrops(3));
}

pub fn raindrops(n: u32) -> String {
    let ver_1 = "Pling".to_string();
    let ver_2 = "Plang".to_string();
    let ver_3 = "Plong".to_string();

    match n {
        n if n % 3 == 0 && n % 5 == 0 && n % 7 == 0 => format!("{}{}{}", ver_1, ver_2, ver_3),
        n if n % 3 == 0 && n % 7 == 0 => format!("{}{}", ver_1, ver_3),
        n if n % 3 == 0 && n % 5 == 0 => format!("{}{}", ver_1, ver_2),
        n if n % 5 == 0 && n % 7 == 0 => format!("{}{}", ver_2, ver_3),
        n if n % 3 == 0 => ver_1,
        n if n % 5 == 0 => ver_2,
        n if n % 7 == 0 => ver_3,
        _ => n.to_string(),
    }
}
