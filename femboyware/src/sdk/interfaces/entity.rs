use crate::netvar;
use crate::sdk::interfaces::base_object::BaseObject;

#[repr(C)]
pub struct BaseEntity {}

impl BaseObject for BaseEntity {}

impl BaseEntity
{
    pub fn health(&self) -> i32
    {
        netvar!(self, "DT_BasePlayer->m_iHealth");
    }
}
