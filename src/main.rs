use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env;
use std::collections::HashMap;

use regex::Regex;


fn count_words( s: String, reg: (&str,&str) ) {
    let expr = Regex::new(reg.1).unwrap();
    let count= expr.find_iter(&s).count();
    println!("{}: {}", reg.0, count);
}

fn counter<R: BufRead> ( reader: &mut R, regexes: HashMap<&str, &str> ) -> Result<(i32), String> {

    let mut total_lines: i32 = 0;
    let mut line = String::from( "" );

    loop{ 
        match reader.read_line( &mut line ) {
            Ok( _ ) => {
                    if line.len() == 0 {
                        break;
                    }

                    line = line.trim().to_string();
                    total_lines += 1;

                    for regex in regexes{
                        count_words( line, regex );
                    }

                    line.clear(); 
            },
            Err( why ) => return Err( why.to_string() )
        };
    

    }
    Ok( ( total_lines ) )
}


fn count_file( file_path: &Path, regexes: HashMap<&str, &str>) -> Result< (i32), String> {

    let file_handle = match File::open( &file_path ) {
        // Parse the result of open, returning an Err()
        // or a file_handle
        Err( why ) => return Err( why.to_string() ),
        Ok( file_handle ) => file_handle
    };

    // On successful opening of the file, create a buffered reader
    let mut reader = BufReader::new( file_handle );

    // Call the counter and return the results.
    let (lines) = counter( &mut reader, regexes )?;
    Ok(lines)

}

/*
 * Main
 *
 * Read filenames from the command line and count the
 * number of words in them.
 *
 */
fn main() {

    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Determine if we have any arguments.
    if args.len() < 2 {

        panic!( "Program arguments missing. Please provide a file name" );            

    } 

    // Get arguments from the command line, skipping the program name
    let files: Vec<String> = Vec::from( &args[1..] );

    let mut regexes: HashMap<&str, &str> = HashMap::new();

    regexes.insert("Allocations", "alloc");
    regexes.insert("Frees", "free");
    regexes.insert("Opens", "fopen");
    regexes.insert("Closes", "flose");
    regexes.insert("Cases", "case");
    regexes.insert("Breaks", "break;");
    regexes.insert("Continues", "continue;");

    // Iterate through file names
    for file_name in files.iter() {
        
        // Turn into a Path
        let path = Path::new( &file_name ); 

        // Execute count_file() on it, parsing the response.
        match count_file( path , regexes) {

            Ok( ( lines) ) => {
                println!("{}\t{} lines", path.display(), lines );
            },
            Err( err ) => {
                panic!("Error - {}", err );
            }

        };
    }
}