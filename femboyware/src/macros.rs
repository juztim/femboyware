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

#[macro_export]
macro_rules! netvar {
    ($self:expr,$netvar:literal) => {
        static mut cache: usize = 0usize;
        unsafe {
            if cache == 0
            {
                cache = $crate::utils::netvars::get($netvar);
                return $self.get(cache);
            }
            return $self.get(cache);
        }
    };
}
