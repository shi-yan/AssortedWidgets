// RGB565 color encoding for terminal minimap
//
// Uses 16-bit RGB565 format (5 bits red, 6 bits green, 5 bits blue)
// to store ANSI colors compactly in the minimap.
//
// Memory: 2 bytes per pixel for color + 1 byte for intensity = 3 bytes/pixel
// vs 4 bytes/pixel for RGBA888

/// Encode RGB888 (24-bit) color to RGB565 (16-bit)
///
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)
/// * `b` - Blue component (0-255)
///
/// # Returns
/// 16-bit RGB565 encoded value
///
/// # Format
/// ```text
/// RGB565: RRRRR_GGGGGG_BBBBB
/// Bit positions:
///   R: bits 11-15 (5 bits)
///   G: bits 5-10  (6 bits)
///   B: bits 0-4   (5 bits)
/// ```
#[inline]
pub fn encode_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r5 = (r >> 3) as u16;  // 8 bits -> 5 bits (keep top 5)
    let g6 = (g >> 2) as u16;  // 8 bits -> 6 bits (keep top 6)
    let b5 = (b >> 3) as u16;  // 8 bits -> 5 bits (keep top 5)

    (r5 << 11) | (g6 << 5) | b5
}

/// Decode RGB565 (16-bit) to RGB888 (24-bit)
///
/// # Arguments
/// * `color` - 16-bit RGB565 encoded color
///
/// # Returns
/// Tuple of (r, g, b) in 8-bit format (0-255 each)
///
/// Note: Decoding loses some precision due to bit reduction.
/// Original 255 red -> 5 bits (31) -> back to 248 (not 255)
#[inline]
pub fn decode_rgb565(color: u16) -> (u8, u8, u8) {
    let r5 = ((color >> 11) & 0x1F) as u8;  // Extract 5 bits
    let g6 = ((color >> 5) & 0x3F) as u8;   // Extract 6 bits
    let b5 = (color & 0x1F) as u8;          // Extract 5 bits

    // Scale back to 8-bit range
    // 5 bits: multiply by 255/31 ≈ 8.226, we use (x << 3) | (x >> 2) for better precision
    // 6 bits: multiply by 255/63 ≈ 4.047, we use (x << 2) | (x >> 4) for better precision
    let r = (r5 << 3) | (r5 >> 2);
    let g = (g6 << 2) | (g6 >> 4);
    let b = (b5 << 3) | (b5 >> 2);

    (r, g, b)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_rgb565() {
        // Test pure colors
        assert_eq!(encode_rgb565(255, 0, 0), 0xF800);    // Pure red
        assert_eq!(encode_rgb565(0, 255, 0), 0x07E0);    // Pure green
        assert_eq!(encode_rgb565(0, 0, 255), 0x001F);    // Pure blue
        assert_eq!(encode_rgb565(255, 255, 255), 0xFFFF); // White
        assert_eq!(encode_rgb565(0, 0, 0), 0x0000);      // Black
    }

    #[test]
    fn test_decode_rgb565() {
        // Test pure colors
        let (r, g, b) = decode_rgb565(0xF800);
        assert!(r >= 248 && r <= 255);  // Red (slight precision loss)
        assert!(g <= 7);                 // Green should be near 0
        assert!(b <= 7);                 // Blue should be near 0

        let (r, g, b) = decode_rgb565(0x07E0);
        assert!(r <= 7);                 // Red should be near 0
        assert!(g >= 252);               // Green (slight precision loss)
        assert!(b <= 7);                 // Blue should be near 0

        let (r, g, b) = decode_rgb565(0x001F);
        assert!(r <= 7);                 // Red should be near 0
        assert!(g <= 7);                 // Green should be near 0
        assert!(b >= 248 && b <= 255);   // Blue (slight precision loss)
    }

    #[test]
    fn test_round_trip() {
        // Test round-trip for various colors
        let test_colors = vec![
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
            (128, 128, 128),
            (255, 128, 64),
            (32, 64, 128),
        ];

        for (r_orig, g_orig, b_orig) in test_colors {
            let encoded = encode_rgb565(r_orig, g_orig, b_orig);
            let (r_dec, g_dec, b_dec) = decode_rgb565(encoded);

            // Allow some precision loss (within ~8 units for 5-bit, ~4 for 6-bit)
            assert!((r_orig as i16 - r_dec as i16).abs() <= 8, "Red mismatch: {} vs {}", r_orig, r_dec);
            assert!((g_orig as i16 - g_dec as i16).abs() <= 4, "Green mismatch: {} vs {}", g_orig, g_dec);
            assert!((b_orig as i16 - b_dec as i16).abs() <= 8, "Blue mismatch: {} vs {}", b_orig, b_dec);
        }
    }

    #[test]
    fn test_ansi_colors() {
        // Test standard ANSI colors (16 colors)
        let ansi_colors = vec![
            (0, 0, 0),           // Black
            (205, 49, 49),       // Red
            (13, 188, 121),      // Green
            (229, 229, 16),      // Yellow
            (36, 114, 200),      // Blue
            (188, 63, 188),      // Magenta
            (17, 168, 205),      // Cyan
            (229, 229, 229),     // White (bright)
        ];

        for (r, g, b) in ansi_colors {
            let encoded = encode_rgb565(r, g, b);
            let (r_dec, g_dec, b_dec) = decode_rgb565(encoded);

            // Verify reasonable precision
            assert!((r as i16 - r_dec as i16).abs() <= 8);
            assert!((g as i16 - g_dec as i16).abs() <= 4);
            assert!((b as i16 - b_dec as i16).abs() <= 8);
        }
    }
}
