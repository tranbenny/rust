use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub struct ImageStats {
    image_name: String,
    file_size_bytes: u32,
    created_time: SystemTime,
    last_modified_time: SystemTime,
}

impl ImageStats {
    fn new(image_name: String, file_size_bytes: u32, created_time: SystemTime, modified_time: SystemTime) -> Self {
        Self {  
            image_name,
            file_size_bytes,
            created_time,
            last_modified_time: modified_time,
        }
    }
    
    pub fn display_stats(&self) {
        println!();
        println!("================================================================================================");
        println!("Image Stats");
        println!("================================================================================================");
        println!("Image: {}, Size: {} bytes", self.image_name, self.file_size_bytes);
        
        // Convert SystemTime to human-readable format
        let created_datetime: DateTime<Utc> = self.created_time.into();
        let modified_datetime: DateTime<Utc> = self.last_modified_time.into();
        
        println!("Created: {}", created_datetime.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Modified: {}", modified_datetime.format("%Y-%m-%d %H:%M:%S UTC"));
    }
}

// Function to create ImageStats from file path
pub fn get_image_stats(file_path: String) -> Result<ImageStats, std::io::Error> {
    let metadata = std::fs::metadata(&file_path)?;
    let file_size_bytes = metadata.len() as u32;
    
    let created_time = metadata.created()?;
    let modified_time = metadata.modified()?;

    Ok(ImageStats::new(file_path, file_size_bytes, created_time, modified_time))
}