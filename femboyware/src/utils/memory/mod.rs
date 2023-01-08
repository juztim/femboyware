use core::slice;

use std::mem;

use windows::{
    core::HSTRING,
    Win32::System::{
        Diagnostics::Debug::IMAGE_NT_HEADERS32, LibraryLoader::GetModuleHandleW,
        SystemServices::IMAGE_DOS_HEADER,
    },
};

pub unsafe fn get_memory_of_module(name: &str) -> &[u8]
{
    let module = GetModuleHandleW(&HSTRING::from(name))
        .unwrap_or_else(|_| panic!("unable to get module {name}"));

    let dos_header: *const IMAGE_DOS_HEADER = mem::transmute(module);
    let nt_header = (mem::transmute::<_, isize>(module) + (*dos_header).e_lfanew as isize)
        as *const IMAGE_NT_HEADERS32;

    slice::from_raw_parts(
        dos_header as *const u8,
        (*nt_header).OptionalHeader.SizeOfImage as usize,
    )
}

#[derive(Debug)]
pub enum FindSignatureError
{
    SignatureNotFoundError,
}

pub fn find_signature_in_slice<'a, T: Eq>(
    slice: &'a [T],
    signature: &[Option<T>],
) -> Result<&'a T, FindSignatureError>
{
    let mut found_bytes = 0;

    for (i, n) in slice.iter().enumerate()
    {
        if let Some(element) = &signature[found_bytes]
        {
            if n == element
            {
                found_bytes += 1;
            }
            else
            {
                found_bytes = 0;
            }
        }
        else
        {
            found_bytes += 1;
        }

        if found_bytes == signature.len()
        {
            println!("{i} {} {found_bytes}", signature.len());
            return Ok(&slice[(i + 1) - signature.len()]);
        }
    }

    Err(FindSignatureError::SignatureNotFoundError)
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn find_signature_in_slice_finds_signature_in_slice_u8()
    {
        let slice = [0x69, 0xde, 0xad, 0xc0, 0xff, 0xee];

        let signature = [Some(0x69), None, None, Some(0xc0), Some(0xff), Some(0xee)];

        let result = find_signature_in_slice(&slice, &signature).unwrap();

        assert_eq!(result as *const _, &slice[0] as *const _)
    }

    #[test]
    fn find_signature_in_slice_finds_signature_in_slice_u8_more_elements()
    {
        let slice = [
            0xba, 0xbe, 0x12, 0x45, 0x69, 0xde, 0xad, 0xc0, 0xff, 0xee, 0x69, 0x90, 0xaa,
        ];

        let signature = [Some(0x69), None, None, Some(0xc0), Some(0xff), Some(0xee)];

        let result = find_signature_in_slice(&slice, &signature).unwrap();

        assert_eq!(result as *const _, &slice[4] as *const _)
    }
}
