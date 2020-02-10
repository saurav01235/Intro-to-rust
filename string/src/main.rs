fn main() {
    let s= "have fun bro";
    let new_s=s.to_string();
    println!("s is {{{}}} and new_s is {{{}}}",s,new_s );
    
    let mut s1=String::from("Saurav ");
    println!("{}",s1 );
    s1.push_str("jain");
    println!("{}",s1 );
////////////////////////////////////
/// /////////////////////////////////
/// ///////////////////////////////////
/// //////////////////////////////////
///

    //format macr0
    let st1=String::from("tic");
    let st2=String::from("tac");
    let st3=String::from("toe");
    let st4=format!("{}-{}-{}",st1,st2,st3);
    println!("{}",st4 );
    println!( );
    



    //iterate over string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    let hello = "Здравствуйте";
    let answer = &hello[0..2];
    println!("{}",hello.len() );
    println!("{}",answer );
}
