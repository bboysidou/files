use std::borrow::Cow;

const FONT_FILES: [&[u8]; 3] = [
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/JetBrainsMono-Regular.ttf"
    )),
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/JetBrainsMono-SemiBold.ttf"
    )),
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/fonts/JetBrainsMono-Bold.ttf"
    )),
];

pub fn embedded_fonts() -> Vec<Cow<'static, [u8]>> {
    FONT_FILES.iter().map(|font| Cow::Borrowed(*font)).collect()
}
