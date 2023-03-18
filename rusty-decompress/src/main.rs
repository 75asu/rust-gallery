use std::fs;
use std::io;

fn main() {
   std::process::exit(decompress()); 
}

fn decompress() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    // IF USER PROVIDES DIFFERENT NO OF ARGS THEN SHOW THE ACTUAL WAY OF PROVIDING INPUTS
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // PICKUP THE NAME OF THE FILE
    let fname = std::path::Path::new(&*args[1]);
    // OPEN THE FILE FROM PATH
    let file = fs::File::open(&fname).unwrap();
    // COLLECT THE FILE IN ZIP ARCHIVE
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // LOOP THORUGH THE ARCHIVE FROM 0 TO THE LENGTH OF THE ARCHIVE
    for i in  0..archive.len() {
        let mut file = archive
                        .by_index(i)
                        .unwrap();
        // CLONING THE PATH SO THAT FILES CAN BE COPIED FROM ZIP TO ANOTHER FOLDER
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        // CHECKING FOR COMMENTS IN THE FILES
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment)
            }
        }
        // MAINTAINING A FOLDER STRUCTURE AFTER UNZIPPING
        if (*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\" ", i, outpath.display());
            // CREATING DIRECTORIES RECURSIVELY FOR ALL THE FILES
            fs::create_dir_all(&outpath).unwrap();
        }
        else {
            // IF IT'S NOT A DIRECTORY 
            println!(
                "File {} extracted to \"{}\" ( {} bytes)",
                i,
                outpath.display(),
                file.size(),
            );
            // CHECKING FOR PARENTS
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            // COPY FILES INTO A PATH AFTER UNZIPPING
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
        // SETTING USER PERMISSIONS FOR ALL THE UNZIPPED FILES
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode(){
                fs::set_permissions(
                    &outpath, fs::Permissions::from_mode(mode)
                ).unwrap();
            }
        }
    }

    return 0;

}