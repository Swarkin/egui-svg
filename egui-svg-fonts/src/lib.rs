//! Convenience crate for the predefined fonts for `egui-svg`.

use egui::{FontData, FontDefinitions, FontFamily};

/// Convenience struct for the predefined fonts.
pub struct DefaultFont {
    /// Font name
    pub name: &'static str,
    /// Font bytes
    pub bytes: &'static [u8],
    /// egui [`FontFamily`]
    pub family: FontFamily,
}

impl DefaultFont {
    /// Default proportional font
    pub const PROPORTIONAL: DefaultFont = DefaultFont {
        name: "Inter",
        bytes: include_bytes!("../../egui-svg-fonts/assets/Inter-Regular.ttf"),
        family: FontFamily::Proportional,
    };

    /// Default monospace font
    pub const MONOSPACE: DefaultFont = DefaultFont {
        name: "AzeretMono",
        bytes: include_bytes!("../../egui-svg-fonts/assets/AzeretMono-Regular.ttf"),
        family: FontFamily::Monospace,
    };

    /// Convenience function to implement the custom font into [`FontDefinitions`].
    pub fn implement(self, f: &mut FontDefinitions) {
        f.font_data.insert(
            self.name.into(),
            std::sync::Arc::new(FontData::from_static(self.bytes))
        );

        f.families.insert(self.family, vec![self.name.into()]);
    }
}

/// Convenience function to implement the custom fonts into [`FontDefinitions`].
pub fn implement(f: &mut FontDefinitions) {
    DefaultFont::PROPORTIONAL.implement(f);
    DefaultFont::MONOSPACE.implement(f);
}
