use rand::Rng;
use std::convert::TryInto;

fn main() {
    
    println!("Hello, world! ");
    let mut vec =vec![0;81];
  

let mut var=0;
while var<9{
fill_box(&(var*9),&(var*3),&mut vec);
var = var+3;
}

    let mut limit = 1;


    for i in 0..81{
    print!("{} ",vec[i]);
    limit=limit+1;
   if limit > 9 
  {
        limit=1;
        println!("");
    }
    }
    println!("");
   
    println!("Hello, world! "); 
}


fn random()->usize{
    rand::thread_rng().gen_range(1,10)
}

fn fill_box(row : &i32,col : &i32,vec : &mut Vec<i32>){
    let limit1 =9;
    let mut rn :i32;
    for i in 0..3{
        for j in 0..3{
            loop{
        rn=random() as i32;
        if is_used(&row,&col,&rn,&vec){
            break;
          }
        }
      // println!("{} {}",count,rn);
        vec.insert((row+(i*limit1)+j+col).try_into().unwrap(),rn as i32);
      }
    }
}


fn is_used(x :&i32,y:&i32,num:&i32,vec: &Vec<i32>) -> bool{
    let limit=9;
    for i in 0..3{
        for j in 0..3{
            println!("{}{}",i,j);
            if vec[(x+(i*limit)+j+y) as usize]==*num{    //((x+i)*limit+(j+y)) as usize
                return false;
            }
        } 
    }
    return true;
}







////////////////////////////////////////////////////////////////////////
/* while count<9 {
    let rn =rand::thread_rng().gen_range(1,10);
   // println!("{} {}",count,rn);
    if !st.contains(&rn.to_string()){
    st=st+&rn.to_string();
    vec.insert(count,rn);
    count=count+1;
    }
    
} */
////////////////////////////////////////////////////////////////////////









///////////////////////////////////////////////////////////////////////
/* let limit1 =9;
let mut rn :i32;
for i in 0..3{
    for j in 0..3{
        loop{
    rn=random() as i32;
    if is_used(&i,&j,&rn,&vec){
        break;
      }
    }
  // println!("{} {}",count,rn);
    vec.insert((i*limit1+j).try_into().unwrap(),rn as i32);
  }
} */
/////////////////////////////////////////////////////////////////////////