// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    let bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    if bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    let version_bytes = &bytes[0..4];
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}
