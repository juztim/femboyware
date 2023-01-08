pub mod patterns;

use log::info;
use log::trace;
use log::warn;
use std::cell::RefCell;

use std::mem;

use crate::utils::memory::{find_signature_in_slice, get_memory_of_module};

pub struct Pattern<'a, T: 'static>
{
    name: &'a str,
    extra: usize,
    relative: bool,
    module: &'a str,
    offsets: Option<&'a [isize]>,
    signature: &'a [Option<u8>],
    result: RefCell<Option<&'static T>>,
}

impl<'a, T> Pattern<'a, T>
{
    const fn new(
        name: &'a str,
        extra: usize,
        relative: bool,
        module: &'a str,
        offsets: Option<&'a [isize]>,
        signature: &'a [Option<u8>],
    ) -> Self
    {
        Self {
            name,
            extra,
            relative,
            module,
            offsets,
            signature,
            result: RefCell::new(None),
        }
    }

    fn try_resolve(&self)
    {
        unsafe {
            let module_mem = get_memory_of_module(self.module);

            trace!("{} base is at {:p}", self.module, module_mem.as_ptr());

            let result = find_signature_in_slice(module_mem, self.signature);

            if result.is_err()
            {
                warn!("pattern {} was not found", self.name);
                return;
            }

            let mut result = result.unwrap() as *const u8;

            trace!("found signature at {:p}", result);

            if let Some(offsets) = self.offsets
            {
                for offset in offsets
                {
                    let pointer = mem::transmute::<_, *const usize>(result.offset(*offset));
                    result = (*pointer) as *const u8;

                    trace!("adding offset {}", offset);
                }
            }

            result = result.offset(self.extra.try_into().unwrap());

            trace!("adding {} extra", self.extra);

            if self.relative
                && (result < module_mem.as_ptr()
                    || result
                        > (module_mem
                            .as_ptr()
                            .offset(module_mem.len().try_into().unwrap())))
            {
                warn!("pattern {} was found out of bounds", self.name);
                return;
            }

            info!(
                "pattern {} found at {:p}, offset from {} = {:#x}",
                self.name,
                result,
                self.module,
                result.offset_from(module_mem.as_ptr())
            );

            *self.result.borrow_mut() = Some(&*(result as *const T));
        }
    }

    pub fn get(&self) -> Option<&'static T>
    {
        let result = *self.result.borrow();
        if result.is_some()
        {
            result
        }
        else
        {
            drop(result);
            self.try_resolve();
            *self.result.borrow()
        }
    }
}

unsafe impl<T> Send for Pattern<'_, T> {}
unsafe impl<T> Sync for Pattern<'_, T> {}
