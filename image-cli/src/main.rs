use std::{collections::HashSet, path::PathBuf};
use image::ImageReader;

mod image_stats;
use image_stats::get_image_stats;

use std::sync::LazyLock;

static ALLOWED_IMAGE_SIZES: LazyLock<HashSet<&str>> = LazyLock::new(|| {
    HashSet::from(["small", "medium", "large"])
});

// Image resize: Resizes one or more images in a source folder to a specified size
// Image stats: Provides some statistics on the image files present in the source folder
// Read in the env parameters where the first argument is a path
fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let cmd = &args[1];
    match cmd.to_lowercase().as_str() {
        "stats" => display_stats(&args),
        "resize" => resize(&args).expect("resize should have succeeded"),
        _ => println!("unknown cmd"),
    }
}

fn display_stats(args: &[String]) {
    // check if the file exists
    let file_path = args[2].as_str();
    if !std::path::Path::new(file_path).exists() {
        eprintln!("{} file does not exist", file_path);
        std::process::exit(1);
    }

    let image_stats_result = get_image_stats(file_path.to_string());
    match image_stats_result {
        Ok(image_stats) => {
            image_stats.display_stats();
        },
        Err(error) => {
            eprintln!("Error generating image stats: {}", error);
        }
    }
}

fn resize(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::SystemTime::now();
    if args.len() != 4 {
        eprintln!("invalid number of arguments");
        std::process::exit(1);
    }

    let image_size = args[2].as_str();
    validate_image_size(image_size);
    let (height, width) = get_image_dimensions(image_size);

    let file_path = args[3].as_str();
    let path = std::path::Path::new(file_path);
    if !path.exists() {
        eprintln!("file path does not exist. Exiting.");
        std::process::exit(1);
    }


    let img = ImageReader::open(file_path)?.decode()?;

    let resized_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);
    // get the file name
    let output_file_name = create_file_output_name(file_path, image_size);
    println!("Creating file at {}", output_file_name);
    let save_result = resized_img.save(output_file_name);
    match save_result {
        Ok(_result) => { 
            println!("Elapsed Time (ms): {}", now.elapsed()?.as_millis());
        },
        Err(error) => eprintln!("failed to save image. {}", error),
    }
    Ok(())
}

fn create_file_output_name(path: &str, size: &str) -> String {
    let mut file_path = PathBuf::from(path);
    let file_name = file_path.file_name();

    match file_name {
        Some(file) => {
            // remove the extension
            let file_name_without_extension = file.to_str().unwrap().strip_suffix(".png").unwrap();
            let output_file_name = format!("{}_{}.png", file_name_without_extension, size);

            file_path.pop();
            file_path.push(output_file_name);
            String::from(file_path.as_path().as_os_str().to_str().unwrap())
        }
        None => { 
            "".to_string()
        }
    }
}

fn validate_image_size(image_size: &str) {
    if !ALLOWED_IMAGE_SIZES.contains(image_size) {
        eprintln!("{} is not a valid image size", image_size);
        std::process::exit(1);
    }
}

fn get_image_dimensions(size: &str) -> (u32, u32) {
    let mut height = 100;
    let mut width = 400;
    match size {
        "small" => {},
        "medium" => {
            height *= 2;
            width *= 2;
        },
        "large" => {
            height *= 4;
            width *= 4;
        }
        _ => {
            println!("unsupported image size")
        },
    }

    (height, width)
}
