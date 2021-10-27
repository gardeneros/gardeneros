mod heap_allocator;

pub fn init() {
	heap_allocator::init_heap();
	heap_allocator::heap_test();
}
