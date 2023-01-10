// https://gitlab.com/KittenPopo/csgo-2018-source/-/blob/main/public/dt_recv.h

use std::ffi::c_void;
use std::os::raw::c_char;


pub type RecvVarProxyFn = fn(data: *const RecvProxy, struct_ptr: *mut c_void, out_ptr: *mut c_void);

pub type ArrayLengthRecvProxyFn =
    fn(struct_ptr: *mut c_void, object_id: i32, current_array_length: i32);

pub type DataTableRecvVarProxyFn =
    fn(prop: *const RecvProp, out_ptr: *mut *mut c_void, data_ptr: *mut c_void, object_id: i32);

#[repr(C)]
pub struct RecvTable
{
    pub p_props: *mut RecvProp,
    pub n_props: i32,
    pub decoder: *const c_void,
    pub table_name: *const c_char,
    pub is_initialized: bool,
    pub is_in_main_list: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecvProp
{
    pub prop_name: *const c_char,
    pub prop_type: PropType,
    pub prop_flags: i32,
    pub buffer_size: i32,
    pub is_inside_array: i32,
    pub extra_data_ptr: *const c_void,
    pub array_prop: *const RecvProp,
    pub array_length_proxy: ArrayLengthRecvProxyFn,
    pub proxy_fn: RecvVarProxyFn,
    pub data_table_proxy_fn: DataTableRecvVarProxyFn,
    pub data_table: *mut RecvTable,
    pub offset: i32,
    pub element_stride: i32,
    pub elements_count: i32,
    pub parent_array_prop_name: *const c_char,
}

#[repr(C)]
pub struct RecvProxy
{
    pub recv_prop: *const RecvProp,
    pub value: Variant,
    pub element_index: i32,
    pub object_id: i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PropType
{
    Int = 0,
    Float,
    Vec,
    VecXY,
    String,
    Array,
    DataTable,
    Int64,
}

#[repr(C)]
pub union VariantData
{
    pub float: f32,
    pub int: i32,
    pub string: *const c_char,
    pub data: *mut c_void,
    pub vector: [f32; 3],
    pub int64: i64,
}

#[repr(C)]
pub struct Variant
{
    pub data: VariantData,
    pub prop_type: PropType,
}
