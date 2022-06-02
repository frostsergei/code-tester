use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env;
use std::collections::HashMap;

use regex::Regex;


fn count_words( s: &String, reg: (&&str,&&str) ) {
    let expr = Regex::new(reg.1).unwrap();
    let count= expr.find_iter(&s).count();
    println!("{}: {}", reg.0, count);
}

fn counter<R: BufRead> ( reader: &mut R, regexes: &HashMap<&str, &str> ) -> Result<i32, String> {

    let mut total_lines: i32 = 0;
    let mut text = String::from( "" );

    loop{ 
        match reader.read_to_string( &mut text ) {
            Ok( _ ) => {
                    if text.len() == 0 {
                        break;
                    }

                    total_lines += 1;

                    for regex in regexes.into_iter(){
                        count_words( &text, regex );
                    }

                    text.clear(); 
            },
            Err( why ) => return Err( why.to_string() )
        };
    

    }
    Ok(total_lines)
}


fn count_file( file_path: &Path, regexes: &HashMap<&str, &str>) -> Result<i32, String> {

    let file_handle = match File::open( &file_path ) {
        Err( why ) => return Err( why.to_string() ),
        Ok( file_handle ) => file_handle
    };

    let mut reader = BufReader::new( file_handle );

    let lines = counter( &mut reader, regexes )?;
    Ok(lines)

}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!( "Program arguments missing. Please provide a file name" );            
    } 

    let files: Vec<String> = Vec::from( &args[1..] );

    let mut regexes: HashMap<&str, &str> = HashMap::new();

    regexes.insert("Allocations", "alloc");
    regexes.insert("Frees", "free");
    regexes.insert("Opens", "fopen");
    regexes.insert("Closes", "flose");
    regexes.insert("Cases", "case");
    regexes.insert("Breaks", "break;");
    regexes.insert("Continues", "continue;");

    for file_name in files.iter() {
        
        let path = Path::new( &file_name ); 

        match count_file( path , &regexes) {

            Ok(lines) => {
                println!("{}\t{} lines", path.display(), lines );
            },
            Err( err ) => {
                panic!("Error - {}", err );
            }

        };
    }
}