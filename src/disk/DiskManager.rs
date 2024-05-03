use std::fs::File;

pub struct  DiskManager {
    heap_file: File,
    next_page_id: u64,


}