use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

struct Selectors {
  code_selector: SegmentSelector,
  tss_selector: SegmentSelector
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub fn init() {
  use x86_64::instructions::segmentation::set_cs;
  use x86_64::instructions::tables::load_tss;
  
  GDT.0.load();
  unsafe { // the functions are marked unsafe because it might be possible to break memory safety by loading invalid selectors
    set_cs(GDT.1.code_selector);
    load_tss(GDT.1.tss_selector);
  }
}

lazy_static! {
  static ref GDT: (GlobalDescriptorTable, Selectors) = {
    // gdt is a relict that was used for memory segmentation before paging became the de facto standard for implementing virtual memory

    // now, the gdt is used to:
    // 1. switch between kernel space and user space, and 
    // 2. load TSS (task state segment) structures
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (gdt, Selectors { code_selector, tss_selector })
  };
}

lazy_static! {
  static ref TSS: TaskStateSegment = {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = { // this double fault stack has no guard page: we shouldn't do anything in the stack that might cause a stack overflow
      const STACK_SIZE: usize = 4096 * 5;
      static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

      let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
      let stack_end = stack_start + STACK_SIZE;
      stack_end
    };
    tss
  };
}
