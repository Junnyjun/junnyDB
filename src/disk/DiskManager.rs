use std::fs::File;

pub struct  DiskManager {
    heap_file: File,
    next_page_id: u64,


}

impl DiskManager {
    pub fn new(heap_file: File) -> Self {
        let next_page_id = heap_file.metadata().unwrap().len() / PAGE_SIZE as u64;
        DiskManager {
            heap_file,
            next_page_id,
        }
    }
    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        page_id
    }
    pub fn fetch_page_data(&mut self, page_id: PageId) -> Page {
        let mut data = vec![0; PAGE_SIZE];
        self.heap_file.seek(SeekFrom::Start(page_id * PAGE_SIZE as u64)).unwrap();
        self.heap_file.read_exact(&mut data).unwrap();
        data
    }
    pub fn write_page_data(&mut self, page_id: PageId, data: &Page) {
        self.heap_file.seek(SeekFrom::Start(page_id * PAGE_SIZE as u64)).unwrap();
        self.heap_file.write_all(data).unwrap();
    }
}