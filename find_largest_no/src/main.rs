#[warn(non_snake_case)]
fn largest(list:&[i32]) -> i32 {
    let mut large=list[0];
    for &i in list.iter(){
        if large < i{
            large=i;
        }
    }
    large
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}