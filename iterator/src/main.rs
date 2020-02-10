//#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    for i in &v1 {
        println!("{}",i )
        
    }

    let v1_iter = v1.iter();
    
    println!("{:?}",v1_iter );
    for i in v1_iter {
        println!("{}",i )
        
    }
    let total: i32 = v1_iter.sum();
    println!("{}",total ); 

    //assert_eq!(total, 6);
}
fn main(){
    iterator_sum();
}