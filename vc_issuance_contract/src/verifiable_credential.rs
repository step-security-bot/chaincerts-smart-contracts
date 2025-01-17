use soroban_sdk::{Env, String, Vec};

use crate::base32;
use crate::storage;

pub fn add_vc(e: &Env, vc_id: &String, mut vcs: Vec<String>) {
    vcs.push_front(vc_id.clone());

    storage::write_vcs(e, &vcs);
}

pub fn generate_id(e: &Env) -> String {
    let random_bytes: [u8; 15] = get_random_bytes(e);
    let mut id = [0u8; 24];

    base32::encode(&mut id, &random_bytes);

    let str_id = core::str::from_utf8(id.as_ref()).unwrap();

    String::from_str(e, str_id)
}

fn get_random_bytes(e: &Env) -> [u8; 15] {
    let mut random_bytes = [0u8; 15];

    for byte in &mut random_bytes {
        let rand_number: u64 = e.prng().gen_range(0..256);
        *byte = rand_number as u8;
    }

    random_bytes
}
