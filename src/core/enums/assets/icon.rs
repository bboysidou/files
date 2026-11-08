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
    Copy => "copy",
    Cut => "cut",
    Desktop => "desktop",
    Document => "doc",
    Download => "download",
    Drive => "drive",
    Edit => "edit",
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
    NewFolder => "newfolder",
    Open => "open",
    Overview => "overview",
    Panel => "panel",
    Paste => "paste",
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
