
use::std::collections::HashMap;
fn main() {
    let mut score=HashMap::new();
    score.insert(String::from("white"),24);
    score.insert(String::from("black"),42);

    println!("{:#?}",score );

    //mapping to hachmap using vector

    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let score1:HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}",score1 );

    //acccessing value from hashmap
    let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let _score = scores.get(&team_name);
   // println!("{}",scores.get("Blue") );

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

//inserting value if key dont have value
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

//Updating a Value Based on the Old Value


let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);


}
