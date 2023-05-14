use std::str;
use digest::DynDigest;
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512};

pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && n & (n - 1) == 0
}

pub fn hash_data(data: &str, hash_function: &str) -> String {
    let mut hasher = select_hasher(hash_function);

    let hash = use_hasher(&mut *hasher, data.as_bytes());
    String::from_utf8(hash.to_vec()).unwrap()
}

fn use_hasher(hasher: &mut dyn DynDigest, data: &[u8]) -> Box<[u8]> {
    hasher.update(data);
    hasher.finalize_reset()
}

fn select_hasher(s: &str) -> Box<dyn DynDigest> {
    match s {
        "sha1" => Box::new(Sha1::default()),
        "sha224" => Box::new(Sha224::default()),
        "sha256" => Box::new(Sha256::default()),
        "sha384" => Box::new(Sha384::default()),
        "sha512" => Box::new(Sha512::default()),
        _ => unimplemented!("unsupported digest: {}", s),
    }
}

pub fn concat_and_hash_list(lst: &mut Vec<String>, hash_function: &str) -> String {
    assert!(lst.len() >= 2, "No transactions to be hashed");

    while lst.len() > 1 {
        let a = lst.remove(0);
        let b = lst.remove(0);
        lst.push(hash_data(&(a + &b), hash_function));
    }

    lst[0].clone()
}

