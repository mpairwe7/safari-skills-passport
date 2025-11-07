use qrcode::QrCode;
use image::Luma;
use base64::{Engine as _, engine::general_purpose};
use common::{AppError, AppResult};

pub fn generate_qr_code(data: &str) -> AppResult<String> {
    // Generate QR code
    let code = QrCode::new(data.as_bytes())
        .map_err(|e| AppError::Internal(format!("Failed to generate QR code: {}", e)))?;

    // Render to image
    let image = code.render::<Luma<u8>>().build();

    // Convert to PNG bytes
    let mut png_bytes = Vec::new();
    image
        .write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .map_err(|e| AppError::Internal(format!("Failed to encode QR code: {}", e)))?;

    // Encode to base64
    let base64_qr = general_purpose::STANDARD.encode(&png_bytes);

    Ok(base64_qr)
}
