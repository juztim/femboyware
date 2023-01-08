use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config
{
    pub executable: String,
    pub filename: String,
    pub signatures: Vec<Signature>,
}

#[derive(Deserialize)]
pub struct Signature
{
    pub name: String,
    pub extra: usize,
    pub relative: bool,
    pub module: String,
    pub offsets: Option<Vec<isize>>,
    pub pattern: String,
    pub result_type: Option<String>,
}
