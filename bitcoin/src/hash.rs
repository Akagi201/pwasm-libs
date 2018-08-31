use sha2::{Digest, Sha256};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Default, Ord, PartialOrd)]
pub struct Hash([u8; 32]);

impl Hash {
  pub fn from_data(data: &[u8]) -> Hash {
    let mut hasher1 = Sha256::default();
    let mut hasher2 = Sha256::default();

    hasher1.input(data);
    let out = hasher1.result();

    hasher2.input(&out);
    let out = hasher2.result();

    let slice = out.as_slice();
    let mut a: [u8; 32] = [0u8; 32];
    a.copy_from_slice(&slice);

    Hash(a)
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
