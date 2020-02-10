#[derive(Debug)]
struct HaveFun {
    name:String,
    id:i32,
    address:String,
}
fn main() {
    let have_fun=HaveFun{name:String::from("saurav jain"),id:1,address:String::from("gwalior")};
    println!("Hello, world!");
    println!("{:#?}",have_fun);

}
