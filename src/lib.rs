#![feature(test)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(alloc, allocator_api)]

extern crate alloc;

use alloc::allocator::Layout;

extern crate linked_list_allocator;
extern crate test;
extern crate slab_allocator;

use std::mem::align_of;
use std::mem::size_of;


const HEAP_SIZE: usize = 2048 * 4096;


#[repr(align(4096))]
struct TestHeap {
    heap_space: [u8; HEAP_SIZE],
}

use test::Bencher;

fn new_linked_list_heap() -> linked_list_allocator::Heap {
    let heap_space = Box::into_raw(Box::new([0u8; HEAP_SIZE]));

    let heap = unsafe { linked_list_allocator::Heap::new(heap_space as usize, HEAP_SIZE) };
    heap
}

fn new_slab_heap() -> slab_allocator::Heap {
    let heap_space = Box::into_raw(Box::new([0u8; HEAP_SIZE]));

    let heap = unsafe { slab_allocator::Heap::new(heap_space as usize, HEAP_SIZE) };
    heap
}

#[bench]
fn allocate_many_small_blocks_linked_list(b: &mut Bencher) {
    let mut heap = new_linked_list_heap();
    b.iter(|| {
        for _ in 0..100000 {
            let layout = Layout::from_size_align(size_of::<usize>() * 2, align_of::<usize>()).unwrap();
            let a = heap.allocate_first_fit(layout.clone());
            let b = heap.allocate_first_fit(layout.clone());
            let c = heap.allocate_first_fit(layout.clone());
            let d = heap.allocate_first_fit(layout.clone());
            let e = heap.allocate_first_fit(layout.clone());
            let f = heap.allocate_first_fit(layout.clone());
            let g = heap.allocate_first_fit(layout.clone());

            unsafe {
                heap.deallocate(a.unwrap(), layout.clone());
                heap.deallocate(b.unwrap(), layout.clone());
                heap.deallocate(c.unwrap(), layout.clone());
                heap.deallocate(d.unwrap(), layout.clone());
                heap.deallocate(e.unwrap(), layout.clone());
                heap.deallocate(f.unwrap(), layout.clone());
                heap.deallocate(g.unwrap(), layout.clone());
            }
        }
    });
}

#[bench]
fn allocate_many_small_blocks_slab(b: &mut Bencher) {
    let mut heap = new_slab_heap();
    b.iter(|| {
        for _ in 0..100000 {
            let layout = Layout::from_size_align(size_of::<usize>() * 2, align_of::<usize>()).unwrap();
            let a = heap.allocate(layout.clone());
            let b = heap.allocate(layout.clone());
            let c = heap.allocate(layout.clone());
            let d = heap.allocate(layout.clone());
            let e = heap.allocate(layout.clone());
            let f = heap.allocate(layout.clone());
            let g = heap.allocate(layout.clone());

            unsafe {
                heap.deallocate(a.unwrap(), layout.clone());
                heap.deallocate(b.unwrap(), layout.clone());
                heap.deallocate(c.unwrap(), layout.clone());
                heap.deallocate(d.unwrap(), layout.clone());
                heap.deallocate(e.unwrap(), layout.clone());
                heap.deallocate(f.unwrap(), layout.clone());
                heap.deallocate(g.unwrap(), layout.clone());
            }
        }
    });
}

#[bench]
fn allocate_multiple_sizes_linked_list_up_to_4096(b: &mut Bencher) {
    let mut heap = new_linked_list_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(32, 16).unwrap();
    let layout_2 = Layout::from_size_align(64, 16).unwrap();
    let layout_3 = Layout::from_size_align(128, 16).unwrap();
    let layout_4 = Layout::from_size_align(256, 16).unwrap();
    let layout_5 = Layout::from_size_align(512, 16).unwrap();
    let layout_6 = Layout::from_size_align(1024, 16).unwrap();
    let layout_7 = Layout::from_size_align(2048, 16).unwrap();
    let layout_8 = Layout::from_size_align(4096, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..150 {
        vec.push(heap.allocate_first_fit(layout_1.clone()));
        vec.push(heap.allocate_first_fit(layout_2.clone()));
        vec.push(heap.allocate_first_fit(layout_3.clone()));
        vec.push(heap.allocate_first_fit(layout_4.clone()));
        vec.push(heap.allocate_first_fit(layout_5.clone()));
        vec.push(heap.allocate_first_fit(layout_6.clone()));
        vec.push(heap.allocate_first_fit(layout_7.clone()));
        vec.push(heap.allocate_first_fit(layout_8.clone()));
    }

    for _ in 0..150 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_8.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_7.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_6.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_5.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_4.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_3.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}

#[bench]
fn allocate_multiple_sizes_slab_up_to_4096(b: &mut Bencher) {
    let mut heap = new_slab_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(32, 16).unwrap();
    let layout_2 = Layout::from_size_align(64, 16).unwrap();
    let layout_3 = Layout::from_size_align(128, 16).unwrap();
    let layout_4 = Layout::from_size_align(256, 16).unwrap();
    let layout_5 = Layout::from_size_align(512, 16).unwrap();
    let layout_6 = Layout::from_size_align(1024, 16).unwrap();
    let layout_7 = Layout::from_size_align(2048, 16).unwrap();
    let layout_8 = Layout::from_size_align(4096, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..150 {
        vec.push(heap.allocate(layout_1.clone()));
        vec.push(heap.allocate(layout_2.clone()));
        vec.push(heap.allocate(layout_3.clone()));
        vec.push(heap.allocate(layout_4.clone()));
        vec.push(heap.allocate(layout_5.clone()));
        vec.push(heap.allocate(layout_6.clone()));
        vec.push(heap.allocate(layout_7.clone()));
        vec.push(heap.allocate(layout_8.clone()));
    }

    for _ in 0..150 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_8.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_7.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_6.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_5.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_4.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_3.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}

#[bench]
fn allocate_multiple_sizes_linked_list_various_sizes(b: &mut Bencher) {
    let mut heap = new_linked_list_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(8192, 16).unwrap();
    let layout_2 = Layout::from_size_align(64, 16).unwrap();
    let layout_3 = Layout::from_size_align(128, 16).unwrap();
    let layout_4 = Layout::from_size_align(256, 16).unwrap();
    let layout_5 = Layout::from_size_align(512, 16).unwrap();
    let layout_6 = Layout::from_size_align(1024, 16).unwrap();
    let layout_7 = Layout::from_size_align(2048, 16).unwrap();
    let layout_8 = Layout::from_size_align(4096, 16).unwrap();
    let layout_9 = Layout::from_size_align(16384, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..40 {
        vec.push(heap.allocate_first_fit(layout_1.clone()));
        vec.push(heap.allocate_first_fit(layout_2.clone()));
        vec.push(heap.allocate_first_fit(layout_3.clone()));
        vec.push(heap.allocate_first_fit(layout_4.clone()));
        vec.push(heap.allocate_first_fit(layout_5.clone()));
        vec.push(heap.allocate_first_fit(layout_6.clone()));
        vec.push(heap.allocate_first_fit(layout_7.clone()));
        vec.push(heap.allocate_first_fit(layout_8.clone()));
        vec.push(heap.allocate_first_fit(layout_9.clone()));
    }

    for _ in 0..40 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_9.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_8.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_7.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_6.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_5.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_4.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_3.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}

#[bench]
fn allocate_multiple_sizes_slab_various_sizes(b: &mut Bencher) {
    let mut heap = new_slab_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(8192, 16).unwrap();
    let layout_2 = Layout::from_size_align(64, 16).unwrap();
    let layout_3 = Layout::from_size_align(128, 16).unwrap();
    let layout_4 = Layout::from_size_align(256, 16).unwrap();
    let layout_5 = Layout::from_size_align(512, 16).unwrap();
    let layout_6 = Layout::from_size_align(1024, 16).unwrap();
    let layout_7 = Layout::from_size_align(2048, 16).unwrap();
    let layout_8 = Layout::from_size_align(4096, 16).unwrap();
    let layout_9 = Layout::from_size_align(16384, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..40 {
        vec.push(heap.allocate(layout_1.clone()));
        vec.push(heap.allocate(layout_2.clone()));
        vec.push(heap.allocate(layout_3.clone()));
        vec.push(heap.allocate(layout_4.clone()));
        vec.push(heap.allocate(layout_5.clone()));
        vec.push(heap.allocate(layout_6.clone()));
        vec.push(heap.allocate(layout_7.clone()));
        vec.push(heap.allocate(layout_8.clone()));
        vec.push(heap.allocate(layout_9.clone()));
    }

    for _ in 0..40 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_9.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_8.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_7.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_6.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_5.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_4.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_3.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}


#[bench]
fn allocate_multiple_sizes_linked_list_over_4096(b: &mut Bencher) {
    let mut heap = new_linked_list_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(8192, 16).unwrap();
    let layout_2 = Layout::from_size_align(12288, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..50 {
        vec.push(heap.allocate_first_fit(layout_1.clone()));
        vec.push(heap.allocate_first_fit(layout_2.clone()));
    }

    for _ in 0..50 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}

#[bench]
fn allocate_multiple_sizes_slab_over_4096(b: &mut Bencher) {
    let mut heap = new_slab_heap();

    b.iter(|| {
    let layout_1 = Layout::from_size_align(8192, 16).unwrap();
    let layout_2 = Layout::from_size_align(12288, 16).unwrap();

    let mut vec = Vec::new();

    for _ in 0..50 {
        vec.push(heap.allocate(layout_1.clone()));
        vec.push(heap.allocate(layout_2.clone()));
    }

    for _ in 0..50 {
        unsafe {
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_2.clone());
            heap.deallocate(vec.pop().unwrap().unwrap(), layout_1.clone());
        }
    }
    });
}