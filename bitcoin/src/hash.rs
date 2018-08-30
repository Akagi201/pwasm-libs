use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, Ord, PartialOrd)]
pub struct Hash([u8; 32]);

impl Hash {
  pub fn from_data(data: &[u8]) -> Hash {
    let mut out = [0u8; 32];
    let mut hasher1 = Sha256::new();
    let mut hasher2 = hasher1;

    hasher1.input(data);
    hasher1.result(&mut out);

    hasher2.input(&out);
    hasher2.result(&mut out);

    Hash(out)
  }

  pub fn from_slice(slice: &[u8; 32]) -> Hash {
    let mut out = [0u8; 32];
    out.copy_from_slice(slice);
    Hash(out)
  }

  pub fn as_slice(&self) -> &[u8] {
    &self.0
  }

  pub fn as_mut_slice(&mut self) -> &mut [u8] {
    &mut self.0
  }
}

pub static ZERO_HASH: Hash = Hash([0; 32]);
