use rand::Rng;
use std::convert::TryInto;

fn main() {
    
    println!("Hello, world! ");
    let mut vec =vec![0;81];
  

let mut var=0;
while var<9{
//fill_box(&(var*9),&(var),&mut vec);
fill_box(&(var),&(var),&mut vec);
var = var+3;
}
/* let mut limit1 = 1;
for i in 0..81{
    if i==27 || i==54{
        for _i in 0..9{
            print!("==")
        }
        println!("===");
    }
    print!("{} ",vec[i]);
    if limit1==3 || limit1==6{  ///////////////
        print!("| ");
    }
    
    limit1=limit1+1;
   if limit1 > 9 
  {
        limit1=1;
        println!("");
    }
    }
    println!(""); */
    /////////////////////////////////////////////////////////////////////////
let mut k=0;
let mut m=3;
fill_remain(&mut vec,&mut k,&mut m);///////////fill remain

    let mut limit = 1;


    for i in 0..81{
        if i==27 || i==54{
            for _i in 0..9{
                print!("==")
            }
            println!("===");
        }
    print!("{} ",vec[i]);
    if limit==3 || limit==6{  ///////////////
        print!("| ");
    }
    limit=limit+1;
   if limit > 9 
  {
        limit=1;
        println!("");
    }
    }
    println!("");
   
    println!("Hello, world! {}",vec[72]); 
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
        vec.insert((row*9+(i*limit1)+j+col).try_into().unwrap(),rn as i32);
      }
    }
}


//////////////////////////////////////////////////////////////////////////////////

fn fill_remain(vec:&mut Vec<i32>,i:&mut i32,j:&mut i32)->bool{
if *j>=9 && *i<9 {
    *i=*i+1;
    *j=0;
}
if *i>=9 && *j>=9{ 
return true; 
}

if *i < 3 
{ 
if *j < 3{ 
    *j = 3;
} 
} 
else if *i < 9-3 
{ 
if j==i{ 
    *j = *j + 3;
} 
} 
else
{ 
if *j == 6 
{ 
    *i = *i + 1; 
    *j = 0; 
    if *i>=9{ 
        return true;
    } 
} 
} 

for num in 1..10 
{ 
if is_safe(&i,&j,&num,&vec) 
{ 
    vec[((*i*9)+*j) as usize] = num; 
    if fill_remain(&mut *vec,&mut (*i),&mut(*j+1)) {
        return true; }

        vec[(*i*9+*j) as usize] = 0; 
} 

} 
return false; 
} 

fn is_safe(i:&i32,j:&i32,num:&i32,vec:&Vec<i32>)-> bool{
    return row_unused(&i,&num,&vec) && 
    col_unused(&j,&num,&vec)  && 
    is_used(&(i-i%3), &(j-j%3),&num,&vec)
}

/////////////////////////////////////////////////////////
fn is_used(x :&i32,y:&i32,num:&i32,vec: &Vec<i32>) -> bool{
    let limit=9;
    for i in 0..3{

        for j in 0..3{
            //println!("{}{}",i,j);
            if vec[(x*limit+(i*limit)+j+y) as usize]==*num{    //((x+i)*limit+(j+y)) as usize
                return false;
            }
     
        } 
    }
    return true;
}
fn row_unused(i:&i32,num:&i32,vec:&Vec<i32>)->bool{
    let tem=i*9;
for k in 0..9  {
if vec[(tem+k) as usize]==*num{
    return false;
}
}
return true;
}
fn col_unused(j:&i32,num:&i32,vec:&Vec<i32>)->bool{
    for k in 0..9  {
    if vec[(k*9+ j) as usize]==*num{
        return false;
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