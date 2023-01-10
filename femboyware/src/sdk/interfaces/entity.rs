use crate::netvar;

#[repr(C)]
pub struct BaseEntity {}

impl BaseEntity
{
    pub fn health(&self) -> i32
    {
        netvar!(self, "DT_BasePlayer->m_iHealth");
    }
}
