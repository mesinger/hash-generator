use uuid::Uuid;
use sha2::{Sha256, Digest};

fn main() {
    let uuid = Uuid::new_v4();
    println!("{:x}", sha2::Sha256::digest(uuid.to_string().as_bytes()));
}
