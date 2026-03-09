use std::env;
use std::fs;
use std::path::Path;

fn walk_path(path: &Path) {
    if path.is_file() {
         println!("{}", path.display());
         return;
    } else if path.is_dir() {
        let _ = fs::read_dir(path).map(|entries| {
            let _ = entries.map(|entry| {
                let _ = entry.map(|p| walk_path(p.path().as_path()));
            });
        });
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
