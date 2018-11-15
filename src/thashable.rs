pub type Hash = [u8; 16];

pub trait THashable {
    fn calc_hash(&self) -> Hash;
}
