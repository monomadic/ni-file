use md5;

pub fn calculate(buffer: &[u8]) -> String {
    format!("{:x}", &md5::compute(buffer))
}