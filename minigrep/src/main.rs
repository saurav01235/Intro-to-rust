use std::env;

use std::process;
use minigrep::Config;

fn main() {
    let args:Vec<String> =env::args().collect();
    //println!("{:?}!",args);
   // let config=parse_config(&args);
   let config =Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
});
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e)=minigrep::run(config){
        println!("Application error {}",e );
        process::exit(1);
    //run(config);
    }
/* let contents = fs::read_to_string(config.filename).expect("something went wront with this file");
println!("----------------------------------------------------------- \n{}",contents); */

}
/* impl Config {
    fn new(args:&[String])->Config{
        if args.len()<3{
            panic!("Not enough arguments");
        }
            let query =args[1].clone();
            let filename =args[2].clone();
            Config{query,filename}
        
        }

} */


/* fn parse_config(args:&[String])->Config{
/*     let query = &args[1];
    let filename = &args[2]; */
    let query =args[1].clone();
    let filename =args[2].clone();
    Config{query,filename}

}
 */