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
- Filter the current folder as you type, or search every name below it (`f`)
- Type a path to jump there (`:` or `Ctrl+L`, `~` for home)
- Context menu on right click or `m`, and a searchable keymap sheet (`?`)
- Seven colour tags stored in the `user.xdg.tags` attribute (the one Dolphin reads); `tag:red` in
  search, or a click on a sidebar dot, lists every tagged file under Home
- Folders refresh by themselves when files change on disk
- USB drives in the sidebar: click to mount and open, eject button to unmount and power off (udisks2)
- Trash view: Enter restores, Delete deletes permanently after a confirmation, Empty Trash in the menu
- Remembers hidden files, sort order, the sidebar, the preview column, the view mode and your open tabs
- Copy, cut, paste, rename, new folder and move to Trash, all with undo and redo; copy and paste work
  with other file managers and apps through the system clipboard (Wayland)
- Copies show live progress in the sidebar and can be cancelled; later operations queue; pasting over
  existing names asks to keep both, skip or replace (replaced items go to the Trash)
- Quick Look for images, folders, source code with syntax colour, rendered Markdown and the first
  page of a PDF (`pdftoppm`, from poppler) (`Space`)
- Preview column beside the list that follows the cursor (`F3`)
- Columns view (`Ctrl+2`): the parent folder on the left, the open folder in the middle, the
  folder under the cursor on the right; click a side column to move there
- Grid view with image thumbnails (`Ctrl+3`, back to the list with `Ctrl+1`), cached in the shared
  freedesktop thumbnail cache (`~/.cache/thumbnails`) that other file managers also use
- Extension badges coloured from a hash of the extension, so every type gets a stable colour
- Opening files with your default applications (`xdg-open`)

## Build

```sh
cargo run --release
```

Needs a recent stable Rust toolchain and the Wayland or X11 development libraries GPUI uses.

## Command line

```sh
files [PATH]           # open a folder, or the folder holding a file
files --select PATH    # open the folder and put the cursor on PATH
files --default        # make files the default file manager (inode/directory)
files --default off    # give the default back to the previous file manager
files --picker         # register files as the file chooser portal backend
files --portal         # run the file chooser backend (started by D-Bus)
```

Only one window runs: launching `files` again opens the path as a new tab in it. While `files` is
the default, it also answers `org.freedesktop.FileManager1`, so "Show in folder" in other apps
opens it.

### File chooser

`files` can stand in for the GTK or KDE dialog when apps ask the desktop portal for a file. Run
`files --picker`: it writes the D-Bus service file for your user and prints the two steps that need
you (copying `files.portal` into `/usr/share/xdg-desktop-portal/portals/` with sudo, and naming
`files` as the `FileChooser` in `~/.config/xdg-desktop-portal/portals.conf`). Open, save, multiple
selection and folder picking are supported. `Enter` picks, `Esc` cancels. The backend exits when its
last dialog closes; D-Bus starts it again on the next request.

## Settings

Stored in `~/.config/files/settings.toml` (or `$XDG_CONFIG_HOME/files/`), saved as you go.
A file that cannot be read is moved to `settings.toml.bak` and the defaults are used.

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
| `f` / `Ctrl+F` | Search names below this folder |
| `T` then `1`–`7` / `T` `0` | Toggle a colour tag / clear colour tags |
| `:` / `Ctrl+L` | Type a path |
| `v` / `Ctrl+A` / `Shift+↓↑` | Select / select all / extend |
| `y` `x` `p` | Copy / cut / paste |
| `r` `F2` | Rename |
| `dd` `Delete` | Move to Trash |
| `Ctrl+Shift+N` | New folder |
| `z` / `Ctrl+Shift+Z` | Undo / redo |
| `Space` | Quick Look (`←` `→` next file, `+` `−` zoom) |
| `F3` | Show or hide the preview column |
| `Ctrl+1` / `Ctrl+2` / `Ctrl+3` | List / columns / grid view |
| `m` / right click | Context menu |
| `Ctrl+Shift+C` | Copy path |
| `?` | Keyboard shortcuts |
| `Ctrl+H` | Show hidden files |
| `F5` | Reload (rarely needed, folders update live) |
| `Ctrl+B` | Collapse the sidebar |

## Licence

GPL-3.0-or-later. See [LICENSE](LICENSE).

The bundled [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) font is licensed under the
SIL Open Font License 1.1, see [assets/fonts/OFL.txt](assets/fonts/OFL.txt).
