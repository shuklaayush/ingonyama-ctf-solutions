use data::pairs;
use solver::solve;

use crate::cypher::evaluate;
use crate::field::ChallengeElement;

mod cypher;
mod data;
mod field;
mod solver;

// fn decode(num: &ChallengeElement) -> String {
//     let binding = num.to_string();
//     let mut num_str = binding.strip_prefix("0x").unwrap().to_string();
//     if num_str.len() % 2 != 0 {
//         num_str.insert_str(0, "0");
//     }
//     (0..num_str.len())
//             .step_by(2)
//             .map(|i| u8::from_str_radix(&num_str[i..i + 2], 16).unwrap_or_default() as char)
//             .collect()
// }
fn main() {
    for (p, c) in pairs() {
        println!("({}, {}),", &p, &c);
    }

    let key = solve();

    let (p, c) = pairs()[0].clone();
    assert_eq!(evaluate(&p, &key), c);

    println!("Found Key! {}", &key);
}
