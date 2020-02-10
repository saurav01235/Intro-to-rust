fn main() {

let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
let w3 = &s[..8];
let w4 = &s[8..];
println!("{}",hello );
println!("{}",world );
println!("{}",w3 );
println!("{}",w4 );
    /* let mut s = String::from("hello_saurav world");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);

    s.clear(); // this empties the String, making it equal to ""
 */
    
}
/* fn first_word(s:&String)->usize{
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        println!("{} {}",i, item);
        if item==b' '{
        return i;
    }

}
s.len()
}
 */