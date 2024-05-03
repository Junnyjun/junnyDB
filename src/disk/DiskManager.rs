use std::fs::File;
use std::ptr::null;

pub struct  DiskManager {
    heap_file: File,
    next_page_id: u64,


}

impl DiskManager {
    pub fn new(heap_file: File) -> Self {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
    }
    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_file_path)?;
        Self::new(heap_file)
    }
}