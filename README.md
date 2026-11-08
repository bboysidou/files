# files

A fast, keyboard-friendly file manager for any Linux desktop, written in Rust on
[GPUI](https://gpui.rs).

Inspired by [Flea](https://github.com/thisisgm/flea). `files` is a from-scratch rewrite that
runs on any Linux desktop instead of a single distribution. It shares no code with Flea, and the
Flea project is not involved in it.

## Status

Early development. Working today:

- Places sidebar built from your XDG folders, collapsible to an icon strip (`Ctrl+B`)
- List view of real folders: folders first, natural sort, sortable columns, sizes, item counts,
  kinds and relative dates
- Tabs, back/forward/up history, clickable path crumbs
- Filter the current folder as you type
- Copy, cut, paste, rename, new folder and move to Trash, all with undo and redo
- Quick Look for images, any text file and folders (`Space`)
- Extension badges coloured from a hash of the extension, so every type gets a stable colour
- Opening files with your default applications (`xdg-open`)

## Build

```sh
cargo run --release
```

Needs a recent stable Rust toolchain and the Wayland or X11 development libraries GPUI uses.

## Keys

| Keys | Action |
| --- | --- |
| `j` `k` / `↓` `↑` | Move the cursor |
| `gg` / `G` | First / last row |
| `Enter` `l` `→` | Open |
| `h` `←` `Backspace` | Parent folder |
| `Alt+←` / `Alt+→` | Back / forward |
| `t` / `w` | New / close tab |
| `1`–`9`, `[` `]` | Go to a tab |
| `/` / `Esc` | Filter this folder / clear the filter |
| `v` / `Ctrl+A` / `Shift+↓↑` | Select / select all / extend |
| `y` `x` `p` | Copy / cut / paste |
| `r` `F2` | Rename |
| `dd` `Delete` | Move to Trash |
| `Ctrl+Shift+N` | New folder |
| `z` / `Ctrl+Shift+Z` | Undo / redo |
| `Space` | Quick Look (`←` `→` next file, `+` `−` zoom) |
| `Ctrl+H` | Show hidden files |
| `F5` | Reload |
| `Ctrl+B` | Collapse the sidebar |

## Licence

GPL-3.0-or-later. See [LICENSE](LICENSE).
