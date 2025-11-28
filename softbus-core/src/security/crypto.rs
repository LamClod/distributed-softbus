//! 加密模块

use bytes::Bytes;
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use crate::{Error, Result};

const NONCE_LEN: usize = 12;

/// 加密管理器
/// 
/// 提供端到端加密功能
pub struct CryptoManager {
    rng: SystemRandom,
}

impl CryptoManager {
    /// 创建新的加密管理器
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// 生成随机密钥
    pub fn generate_key(&self) -> Result<[u8; 32]> {
        let mut key = [0u8; 32];
        self.rng.fill(&mut key)
            .map_err(|_| Error::Encryption("Failed to generate key".to_string()))?;
        Ok(key)
    }

    /// 加密数据
    pub fn encrypt(&self, key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|_| Error::Encryption("Failed to generate nonce".to_string()))?;

        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| Error::Encryption("Invalid key".to_string()))?;
        
        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut sealing_key = SealingKey::new(unbound_key, CounterNonce::new(nonce_bytes));

        let mut in_out = plaintext.to_vec();
        sealing_key.seal_in_place_append_tag(Aad::empty(), &mut in_out)
            .map_err(|_| Error::Encryption("Encryption failed".to_string()))?;

        // 将nonce和密文组合
        let mut result = Vec::with_capacity(NONCE_LEN + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);

        Ok(result)
    }

    /// 解密数据
    pub fn decrypt(&self, key: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < NONCE_LEN {
            return Err(Error::Encryption("Invalid ciphertext".to_string()));
        }

        let (nonce_bytes, encrypted_data) = ciphertext.split_at(NONCE_LEN);
        
        let unbound_key = UnboundKey::new(&AES_256_GCM, key)
            .map_err(|_| Error::Encryption("Invalid key".to_string()))?;
        
        let mut nonce_array = [0u8; NONCE_LEN];
        nonce_array.copy_from_slice(nonce_bytes);
        let mut opening_key = OpeningKey::new(unbound_key, CounterNonce::new(nonce_array));

        let mut in_out = encrypted_data.to_vec();
        let plaintext = opening_key.open_in_place(Aad::empty(), &mut in_out)
            .map_err(|_| Error::Encryption("Decryption failed".to_string()))?;

        Ok(plaintext.to_vec())
    }
}

impl Default for CryptoManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 简单的计数器nonce序列
struct CounterNonce {
    nonce: [u8; NONCE_LEN],
}

impl CounterNonce {
    fn new(nonce: [u8; NONCE_LEN]) -> Self {
        Self { nonce }
    }
}

impl NonceSequence for CounterNonce {
    fn advance(&mut self) -> std::result::Result<Nonce, Unspecified> {
        Ok(Nonce::assume_unique_for_key(self.nonce))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let manager = CryptoManager::new();
        let key = manager.generate_key().unwrap();
        
        let plaintext = b"Hello, World!";
        let ciphertext = manager.encrypt(&key, plaintext).unwrap();
        let decrypted = manager.decrypt(&key, &ciphertext).unwrap();
        
        assert_eq!(plaintext, decrypted.as_slice());
    }
}
