use crate::utils::memory;
use log::trace;
use std::fmt;

pub struct VmtHook<T: 'static + Clone>
{
    vmt_entry: &'static mut T,
    original_pointer: Option<T>,
}

impl<T: Clone> VmtHook<T>
{
    pub unsafe fn new(class: *mut usize, vmt_index: usize) -> Self
    {
        Self {
            vmt_entry: (**(class as *const *const *mut T))
                .offset(vmt_index.try_into().unwrap())
                .as_mut()
                .unwrap(),
            original_pointer: None,
        }
    }

    pub unsafe fn hook(&mut self, func: T)
    {
        if let None = self.original_pointer
        {
            trace!("hooking {:p} with {:p}", self.vmt_entry, &func);

            self.original_pointer = Some(self.vmt_entry.clone());

            memory::modify_protected(self.vmt_entry, func);
        }
    }

    pub unsafe fn unhook(&mut self)
    {
        if let Some(pointer) = &self.original_pointer
        {
            memory::modify_protected(self.vmt_entry, pointer.clone());
            self.original_pointer = None;
        }
    }

    pub unsafe fn get_original(&self) -> &Option<T>
    {
        &self.original_pointer
    }
}

impl<T: Clone> Drop for VmtHook<T>
{
    fn drop(&mut self)
    {
        unsafe { self.unhook() };
    }
}

impl<T: Clone> fmt::Debug for VmtHook<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        f.write_str("VmtHook")
    }
}
