use std::{path::Path, fs::{self, File}, io::{Read, Write}};

use uuid::Uuid;


pub const UPLOAD_FOLDER:&str = "./network/upload/";
pub const STREAM_FOLDER:&str = "./network/stream/";
pub const CHUNK_SIZE: usize = 250;


pub struct NetworkFolder {}


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
        create_op_files(folder_path);
        self.chunk_file(path, folder_path);
    }

    fn chunk_file(
        &mut self, 
        filepath: &Path, 
        upload_path: &Path
    ) {
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
            chunks.push(name)
        }

        println!("{:?}", chunks);
    }


}


fn create_op_files(path: &Path) {
    let chunk_folder = path.join(Path::new("./chunks"));
    let content_folder = path.join(Path::new("/.content"));

    match fs::create_dir_all(&chunk_folder) {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e.to_string())
    };

    match fs::create_dir_all(&content_folder) {
        Ok(_) => (),
        Err(e) => log::error!("{:?}", e.to_string())
    };
}
