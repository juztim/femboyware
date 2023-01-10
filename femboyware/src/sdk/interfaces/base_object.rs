// C++ thisptr
pub trait BaseObject
{
    fn get<T>(&self, offset: usize) -> T
    {
        unsafe { ((self as *const Self as *const () as usize + offset) as *mut T).read() }
    }
}