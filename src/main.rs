mod decimal;
use crate::decimal::Decimal;

fn main() {
    let d = Decimal::new(2_147_483_648, 0, 0, 10).unwrap();
    println!("is_zero: {:?}", d.is_zero());
    println!("is_positive: {:?}", d.is_positive());
}
