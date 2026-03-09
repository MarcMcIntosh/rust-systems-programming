use std::env;
use std::fs;
use std::path::Path;


fn walk_path(path: &Path) {
    if path.is_file() {
         println!("{}", path.display());
         return;
    } else if path.is_dir() {
    
        match fs::read_dir(path) {
            Err(err) => println!("{}", err),
            Ok(entries) => entries.for_each(|entry| {
                match entry {
                    Ok(p) => walk_path(p.path().as_path()),
                    Err(_) => (),       
                };
            }), 
        }
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    if !path.exists() {
        println!("Path {} does not exist", path.display());
        return;
    }

    walk_path(path);
}
