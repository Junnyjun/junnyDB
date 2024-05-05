pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: bool,
}

pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>
}

pub struct BufferPool {
    pool: Vec<Frame>,
    next_victim_index: BufferId
}