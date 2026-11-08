macro_rules! icons {
    ($($variant:ident => $file:literal),+ $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Icon {
            $($variant),+
        }

        impl Icon {
            pub const ALL: &'static [Icon] = &[$(Icon::$variant),+];

            pub fn path(self) -> &'static str {
                match self {
                    $(Icon::$variant => concat!("icons/", $file, ".svg")),+
                }
            }

            pub fn bytes(self) -> &'static [u8] {
                match self {
                    $(Icon::$variant => include_bytes!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/assets/icons/",
                        $file,
                        ".svg"
                    ))),+
                }
            }
        }
    };
}

icons! {
    Back => "back",
    Columns => "cols",
    Desktop => "desktop",
    Document => "doc",
    Download => "download",
    Drive => "drive",
    Eye => "eye",
    Dual => "dual",
    Filter => "filter",
    Folder => "folder",
    Forward => "fwd",
    Grid => "grid",
    Home => "home",
    Image => "image",
    Keys => "keys",
    List => "list",
    Music => "music",
    Overview => "overview",
    Panel => "panel",
    Plus => "plus",
    Search => "search",
    Sidebar => "side",
    Sliders => "sliders",
    Star => "star",
    Trash => "trash",
    Up => "up",
    Video => "video",
    Close => "x",
}
