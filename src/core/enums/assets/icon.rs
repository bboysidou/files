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
    Eject => "eject",
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
    Network => "network",
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
    Terminal => "terminal",
    Trash => "trash",
    Up => "up",
    Usb => "usb",
    Video => "video",
    Close => "x",
}
