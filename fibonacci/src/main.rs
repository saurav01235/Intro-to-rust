use std::io;
fn main() {
    println!("enter the number" );
    let mut line=String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");
    let mut line:u32=line.trim().parse().expect("please type a number");
    /* if line == 1{
    print!("0 ");
    }
    if line >1 && line <3{
    print!("1 ");
    } */
    print!("0 ");
    print!("1 ");
    let mut n1:u32=0;
    let mut n2:u32=1;
    while line>2{
        print!("{} ",n1+n2);
        let tem=n1;
        n1=n2;
        n2=tem+n2;
        line=line-1;
    }

}
//println!("Hello, world!");