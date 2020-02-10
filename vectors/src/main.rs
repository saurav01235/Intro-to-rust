#[allow(dead_code)]
fn main() {
   let v:Vec<i32>=Vec::new();
  // let _v1=vec![1,2,3];
   println!("{:?}",v );
   {
    let v1=vec![1,2,3];
   println!("{:?}",v1 );
   }
   //println!("{:?}",v1 ); //out of scope 

   ///////////reading the value
   let mut v2=vec![13,23,33,43,53];
   let third=&v2[2];
   println!("third element is {}",third);
    v2.push(79);
    //println!("third element is {}",third); // wont run duw to immutable borrow

   match v2.get(3){
       Some(t)=> println!("Fourth element is {}",t),
       None => println!("there is no Fourth element"),
   }

   println!("{:?}",v2 ); //accesing elemtn of vector v2



   //iteration over vector
   for i in &v2{     // using refernce beacouse of ownership
       println!("{}",i );
   }
   println!("{:?}",v2 );


   let mut v4 = vec![100, 32, 57];
for i in &mut v4 {
    *i += 50;
}
println!("{:?}",v4 );

}
