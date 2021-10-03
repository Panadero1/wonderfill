
/// Only use this is you don't want to retrieve the names that are generated
pub struct SerialNamer {
    num: u32,
}
impl SerialNamer {
    pub fn new() -> SerialNamer {
        SerialNamer {
            num: 0
        }
    }
    pub fn gen_name(&mut self) -> String {
        self.num += 1;
        format!("{}", self.num)
    }
}