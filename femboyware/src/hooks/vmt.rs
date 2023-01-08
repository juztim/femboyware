use core::alloc::GlobalAlloc;
use windows::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_READWRITE, PAGE_READONLY};

unsafe impl Sync for VMTHook {}
unsafe impl Send for VMTHook {}
pub struct VMTHook
{
    base:     *mut usize,
    pub og_table: Vec<usize>,
    table:    Vec<usize>,
}

impl VMTHook
{
    pub fn default() -> Self
    {
        Self {
            base:     std::ptr::null_mut(),
            og_table: Vec::new(),
            table:    Vec::new(),
        }
    }

    pub fn init(&mut self, base: *mut usize)
    {
        let mut og_table: Vec<usize> = Vec::new();
        let mut fn_count = 0isize;
        let base = base as *mut *mut usize;

        unsafe {
            while base.read().offset(fn_count).read() > 0
            {
                og_table.push(base.read().offset(fn_count).read());

                fn_count += 1;
            }
        }
        self.base = base as _;
        self.og_table = og_table;
        self.table = alloc::vec![0; fn_count as usize];
    }

    pub fn hook_func(&mut self, idx: usize, hook: usize) -> &Self
    {
        self.table[idx] = hook;

        unsafe {
            let base = self.base as *mut *mut usize;
            let mut old_protection = PAGE_READONLY; // idk what the fuck the default is
            VirtualProtect(
                base.read().add(idx) as _,
                4,
                PAGE_READWRITE,
                &mut old_protection,
            );

            base.read().add(idx).write(hook);
            VirtualProtect(
                base.read().add(idx) as _,
                4,
                old_protection,
                std::ptr::null_mut(),
            );
        }
        self
    }

    pub fn destroy(&mut self, idx: usize) { self.hook_func(idx, self.og_table[idx]); }
    pub fn original(&self, idx: isize) -> usize { self.og_table[idx as usize] }
}