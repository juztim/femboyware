#[macro_export]
macro_rules! interface {
    ($name:literal) => {{
        *$crate::sdk::interfaces::INTERFACES
            .read()
            .get($name)
            .unwrap() as *mut usize
    }};

    ($name:literal, $t:ty) => {{
        *$crate::sdk::interfaces::INTERFACES
            .read()
            .get($name)
            .unwrap() as *mut $t
    }};
}

#[macro_export]
macro_rules! interface_ref {
    ($name:literal,$t:ty) => {{
        #[allow(unused_unsafe)]
        unsafe {
            (*$crate::sdk::interfaces::INTERFACES
                .read()
                .get($name)
                .unwrap() as *mut $t)
                .as_ref()
                .unwrap()
        }
    }};
}
