use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use uuid::Uuid;

// md5
pub fn md5(data: &str) -> String {
    let mut h = Md5::new();
    h.input_str(data);
    h.result_str()
}

// sha1
pub fn sha1(data: &str) -> String {
    let mut h = Sha1::new();
    h.input_str(data);
    h.result_str()
}

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}









