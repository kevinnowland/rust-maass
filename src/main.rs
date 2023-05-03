mod decimal;
use crate::decimal::Decimal;

fn main() {
    let d = Decimal::new(1, 0, 0, 10).unwrap();
    println!("is_zero: {:?}", d.is_zero());
    println!("is_positive: {:?}", d.is_positive());
    println!("is_negative: {:?}", d.is_negative());
    println!("is_approx_zero: {:}?", d.is_approx_zero());
    println!("is_approx_positive: {:?}", d.is_approx_positive());
    println!("is_approx_negative: {:?}", d.is_approx_negative());
}
