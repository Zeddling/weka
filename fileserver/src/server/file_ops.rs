use futures::SinkExt;
use serde::{Deserialize, Serialize};
use std::{path::Path, fs::{self, File}, io::{Read, Write}};

use uuid::Uuid;


pub const UPLOAD_FOLDER:&str = "./network/upload/";
pub const STREAM_FOLDER:&str = "./network/stream/";
pub const CHUNK_SIZE: usize = 256000;


pub struct NetworkFolder {}

/**
 * Represents information about an uploaded file.
 */
#[derive(Debug, Deserialize, Serialize)]
pub struct FileInfo {
    chunks: Vec<String>,
    folder: String,
    name: String,
}

impl NetworkFolder {
    pub fn create() -> Self {
        let upload = Path::new(UPLOAD_FOLDER);
        let stream = Path::new(STREAM_FOLDER);
        fs::create_dir_all(upload).unwrap();
        fs::create_dir_all(stream).unwrap();

        log::info!("Network folders created");
        NetworkFolder {}
    }

    pub fn upload(&mut self, filename: &str) {
        let path = Path::new(filename);

        let mut file = match fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                log::error!("{:?}", e.to_string());
                panic!()
            },
        };

        let folder_name = Uuid::new_v4().to_string();
        let folder = format!("{}{}", UPLOAD_FOLDER, folder_name);
        let folder_path = Path::new(
            folder.as_str()
        );

        match fs::create_dir_all(&folder_path) {
            Ok(_) => log::info!("Creating {:?}", folder),
            Err(e) => log::error!("{:?}", e.to_string())
        };
        create_op_folders(folder_path);
        let chunks = self.chunk_file(path, folder_path);

        let original_file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap();

        let fileinfo = FileInfo {
            chunks,
            folder: folder,
            name: original_file_name,
        };

        self.save_chunks_info(&fileinfo);
    }

    /**
     * Takes a file and splits it into 256kb chunks.
     */
    fn chunk_file(
        &mut self, 
        filepath: &Path, 
        upload_path: &Path
    ) -> Vec<String> {
        let mut file = File::open(filepath).unwrap();
        let mut chunks: Vec<String> = Vec::new();

        let mut buffer = vec![0; CHUNK_SIZE];
    
        loop {
            let count = file.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            
            let name = Uuid::new_v4().to_string();
            let chunk_path = upload_path
                .join(
                    Path::new("chunks/")
                ).join(
                    Path::new(name.as_str())
                );
            
            
            let mut chunk = File::create(chunk_path).unwrap();

            match chunk.write_all(&buffer[..count]) {
                Ok(_) => (),
                Err(e) => log::error!("{:?}", e.to_string())
            };
            chunks.push(name);
        }

        return chunks;
    }

    /**
     * Reconstructs the chunks into the original file.
     * TODO: To be refactored. Use fileinfo.folder to set
     * path of chunks.
     */
    pub fn reconstruct_file(&self, fileinfo: &FileInfo) {
        let folder_name = Uuid::new_v4().to_string();
        let path = format!(
            "{}/content/{}", 
            STREAM_FOLDER, 
            fileinfo.name
        );
        let chunks_folder = format!(
            "{}chunks/", STREAM_FOLDER
        );
        let mut original = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(path)
            .unwrap();

        for chunk in fileinfo.chunks.clone() {
            let content = fs::read(
                format!(
                    "{}{}",
                    chunks_folder,
                    chunk
                )
            ).unwrap();
            original.write_all(&content).unwrap();
            original.flush().unwrap();
        }
    }

    /**
     * Save chunks information. Currently saves info in a file for testing
     */
    pub fn save_chunks_info(&self, fileinfo: &FileInfo) {
        let mut res = File::create("result.json").unwrap();
        let json = serde_json::to_string(fileinfo).unwrap();
        res.write(json.as_bytes()).unwrap();
    }

}


/**
 * Creates the chunks and content folders
 * for each folder belonging to an individual file
 */
fn create_op_folders(path: &Path) {
    let chunk_folder = path.join(Path::new("./chunks"));
    let content_folder = path.join(Path::new("./content"));

    match fs::create_dir_all(&chunk_folder) {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e.to_string())
    };

    match fs::create_dir_all(&content_folder) {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e.to_string())
    };
}
