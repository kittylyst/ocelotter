use std::path::Path;
use std::fs::File;
use zip::ZipArchive;
use std::io::{Read, Seek};
use zip::result::ZipResult;

pub fn file_to_bytes(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    File::open(path).and_then(|mut file| {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}


pub struct ZipFiles<R: Read + Seek> {
    i: usize,
    archive: ZipArchive<R>,
}

impl<R: Read + Seek> Iterator for ZipFiles<R> {
    type Item = ZipResult<(String, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i < self.archive.len() {
            self.i = i + 1;
            Some(self.archive.by_index(i).and_then(|mut file| {
                let mut content = vec![];
                file.read_to_end(&mut content)?;
                Ok((file.name().to_owned(), content))
            }))
        } else {
            None
        }
    }
}

impl ZipFiles<File> {
    pub fn new(file_name: &str) -> ZipFiles<File> {
        let file = File::open(&file_name)
            .expect(&format!("Couldn't open file {}", &file_name));

        let archive = ZipArchive::new(file)
            .expect(&format!("Problem reading archive {}", &file_name));

        ZipFiles { i: 0, archive }
    }
}
