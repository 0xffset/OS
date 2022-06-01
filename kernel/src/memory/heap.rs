/// Where the kernel heap starts
const HEAP_START: usize = 0x_5555_AAAA_0000;
/// Size of the kernel heap
const HEAP_SIZE: usize = 16 * 1024 * 1024; // 16 MiB

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size2MiB,
    },
    VirtAddr,
};

use super::{allocator::ALLOCATOR, frame_allocator::BitmapFrameAllocator};

/// maps the kernels heap memory area to physical addresses
pub fn init_heap(
    mapper: &mut impl Mapper<Size2MiB>,
    frame_allocator: &mut BitmapFrameAllocator,
) -> Result<(), MapToError<Size2MiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // actually map all frames and exit on error
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
