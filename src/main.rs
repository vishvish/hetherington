use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The command to look for
    command: String,
    /// The path to the directory/folder to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    // print out the command
    println!("Command: {}", args.command);

    // checks if command is ingest and the path is a directory
    if args.command == "ingest" && args.path.is_dir() {
        // recurse through the directory
        recurse(&args.path);
    } else {
        // print out the path
        println!("{:?}", args.path);
    }
}

// recurse through a folder
fn recurse(path: &std::path::PathBuf) {
    // create an array to hold all paths containing potential images
    let mut paths: Vec<std::path::PathBuf> = Vec::new();

    // get the contents of the folder
    let contents = std::fs::read_dir(path).unwrap();
    // iterate through the contents
    for entry in contents {
        // get the entry
        let entry = entry.unwrap();

        // if file is an image, add it to the paths array
        if is_image(&entry.path()) {
            paths.push(entry.path());
        }

        // get the path
        let path = entry.path();
        // check if the path is a directory
        if path.is_dir() {
            // recurse through the directory
            recurse(&path);
        }
    }
    // print out every image path found
    for path in paths.iter() {
        println!("{:?}", path);
    }

    // store all these paths in the db
    let db = setup_db();
    store_paths(&db, paths);

    // print out the number of images in the db
    println!("Number of images: {}", get_num_images(&db));
}

// check filename for image extension
fn is_image(path: &std::path::PathBuf) -> bool {
    // get the filename as lowercase
    let filename = path.file_name().unwrap().to_str().unwrap().to_lowercase();

    // create a list of valid image extensions
    let extensions = [".jpg", ".jpeg", ".dng", ".pef", ".nef", ".cr2", ".raf", ".tif", ".tiff"];

    // check if the filename ends in one of the valid extensions
    for extension in extensions.iter() {
        if filename.ends_with(extension) {
            return true;
        }
    }
    // run out of extensions
    return false;
}

// set up sled database
fn setup_db() -> sled::Db {
    let path = "hetherington_db";

    // open the database
    let db = sled::open(path).unwrap();
    // return the database
    return db
}

// store all paths in the database
fn store_paths(db: &sled::Db, paths: Vec<std::path::PathBuf>) {
    // iterate through the paths
    for path in paths.iter() {
        // convert the path to a string
        let path = path.to_str().unwrap();
        // store the path in the database
        db.insert(path, path).unwrap();
    }
}

// get number of images in the database
fn get_num_images(db: &sled::Db) -> usize {
    // get the number of images
    let num_images = db.len();
    // return the number of images
    return num_images;
}
