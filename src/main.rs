use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dir name comparator", about="Compares the names of all directories within the working directory and shows a list of similar names.")]
struct CliArgs {
    dir: PathBuf,
    #[structopt( default_value = "90")]
    threshold: i32,
}

fn main() -> Result<(),std::io::Error> {
    let args = CliArgs::from_args();
    let current_dir = args.dir;
    let distance_threshold = args.threshold;
    let mut directories: Vec<PathBuf> = Vec::new();
    let mut similars: Vec<(&str, &str, i32)> = Vec::new();

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let filetype = metadata.file_type();
        if filetype.is_dir() {
            println!("{:?}", &path);
            directories.push(path);
        }
    }


    if directories.len() > 0{
        let directories_iter = directories.iter();
        for (_, dir1) in directories_iter.enumerate() {
            let directories_iter2 = directories.iter();
            for(_, dir2) in directories_iter2.enumerate() {
                if dir1.file_name().is_some() && dir2.file_name().is_some() {
                    let name1 = dir1.file_name().unwrap().to_str();
                    let name2 = dir2.file_name().unwrap().to_str();
                    
                    let dist = hamming_distance(&name1.unwrap(), &name2.unwrap()) / name1.unwrap().len() as f32;
                    let round_dist = dist.round() as i32;
                    if  round_dist >= distance_threshold {
                        similars.push((&name1.unwrap(), &name2.unwrap(), round_dist))
                    }
                }
            }
        }
    }

    if similars.len() == 0 {
        println!("No directories found with similarity threshold of {:?}", distance_threshold);
        return Ok(());
    }

    for pair in similars {
        println!("{} is {}% similar to {}", pair.0, pair.2, pair.1)
    };
    return Ok(());
}

fn hamming_distance(str1: &str, str2: &str) -> f32 {
    let mut dist_count: i32 = 0;
    for c in str1.chars() {
        for c2 in str2.chars() {
            if c != c2 {
                dist_count+=1;
            }
        }
    }

    return dist_count as f32;
}