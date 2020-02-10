#[allow(dead_code)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny =>{ println!("1");
        1
    },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::_ =>51,
    }
}
fn main(){
    value_in_cents(Coin::Penny);
    
}