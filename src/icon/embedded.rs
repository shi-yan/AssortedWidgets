use rust_embed::RustEmbed;
use std::collections::HashMap;

/// Embedded icon assets (font and mapping)
#[derive(RustEmbed)]
#[folder = "icons/"]
pub struct IconAssets;

impl IconAssets {
    /// Load icon.woff2 font file
    pub fn icon_font() -> Vec<u8> {
        IconAssets::get("icon.woff2")
            .expect("icon.woff2 not found in embedded assets")
            .data
            .to_vec()
    }

    /// Load and parse icon_mapping.json
    pub fn icon_mapping() -> HashMap<String, char> {
        let json_data = IconAssets::get("icon_mapping.json")
            .expect("icon_mapping.json not found in embedded assets")
            .data;

        let mapping: HashMap<String, String> =
            serde_json::from_slice(&json_data).expect("Invalid icon mapping JSON format");

        // Convert hex strings to Unicode characters
        mapping
            .into_iter()
            .map(|(id, hex)| {
                let codepoint = u32::from_str_radix(&hex, 16)
                    .unwrap_or_else(|_| panic!("Invalid hex codepoint '{}' for icon '{}'", hex, id));
                let character = char::from_u32(codepoint)
                    .unwrap_or_else(|| panic!("Invalid Unicode codepoint U+{} for icon '{}'", hex, id));
                (id, character)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_icon_font() {
        let font_data = IconAssets::icon_font();
        assert!(!font_data.is_empty(), "Icon font should not be empty");
        // WOFF2 magic number check
        assert_eq!(&font_data[0..4], b"wOF2", "Should be valid WOFF2 file");
    }

    #[test]
    fn test_load_icon_mapping() {
        let mapping = IconAssets::icon_mapping();
        assert!(!mapping.is_empty(), "Icon mapping should not be empty");

        // Verify some known icons
        assert!(mapping.contains_key("search"), "Should have 'search' icon");
        assert!(mapping.contains_key("home"), "Should have 'home' icon");

        // Verify search icon codepoint (e8b6 = U+E8B6)
        if let Some(&ch) = mapping.get("search") {
            assert_eq!(ch as u32, 0xe8b6, "Search icon should be U+E8B6");
        }
    }
}
