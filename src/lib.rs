// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
        // Remove 0x prefix if present
        let tx_hex = raw_tx_hex.strip_prefix("0x").unwrap_or(raw_tx_hex);
    
        // Check if hex string is long enough (at least 8 characters for 4 bytes)
        if tx_hex.len() < 8 {
            return Err("Transaction data too short".to_string());
        }
        
        // Extract first 8 characters (4 bytes)
        let version_hex = &tx_hex[0..8];
        
        // Validate hex characters
        if !version_hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Hex decode error".to_string());
        }
        
        // Parse hex string into bytes
        let mut bytes = Vec::new();
        for i in (0..8).step_by(2) {
            let byte_str = &version_hex[i..i+2];
            let byte = u8::from_str_radix(byte_str, 16)
                .map_err(|_| "Failed to parse hex")?;
            bytes.push(byte);
        }
        
        // Convert little-endian bytes to u32
        let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        
        Ok(version)
    
}


