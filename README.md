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
- Search inside files: start the search with `text:` (for example `text:TODO`); text files up to
  2 MB are scanned in the background, binary files skipped, and each hit shows its folder, the
  first matching line and how many more matches the file has, scanned on up to 8 threads; the
  preview column and Quick Look list every matching line with its line number. Typing again
  cancels the running scan
- Type a path to jump there (`:` or `Ctrl+L`, `~` for home); network locations work too (`sftp://`, `smb://`,
  `ftp://`, `dav://` or `user@host:/path`), mounted with `gio mount` (gvfs; each protocol needs its
  gvfs backend). A server that asks for a login opens a user/password dialog (plus domain for
  `smb://`); the password is passed to gio once and never saved. Mounted shares are listed under
  Network in the sidebar, with a button to disconnect. A server you connect to is remembered
  there (in `settings.toml`, never its password): click it (or `Ctrl+0`, then `Enter`) to connect
  again, `×` or `x` forgets it, each entry carries a protocol badge (SFTP, FTP, FTPS, SMB, DAV, DAVS, NFS, one colour
  each) after a key for encrypted protocols or a red warning triangle for FTP, DAV and NFS, the pencil or `e` opens an editor for its name and address (a new name is
  saved at once, a new address once it connects; the sidebar shows the name). The pencil and `×`
  appear on hover or under the keyboard cursor. `Ctrl+K` opens the path bar with `sftp://` ready to type a server
- Context menu on right click or `m`, and a searchable keymap sheet (`?`)
- Seven colour tags stored in the `user.xdg.tags` attribute (the one Dolphin reads); `tag:red` in
  search, or a click on a sidebar dot, lists every tagged file under Home
- Folders refresh by themselves when files change on disk
- USB drives in the sidebar: click to mount and open, eject button to unmount and power off (udisks2)
- Trash view: Enter restores, Delete deletes permanently after a confirmation, Empty Trash in the menu
- Settings panel (sliders button, `,` or `Ctrl+,`): hidden files and whether they sort last, sort
  key (name, size, kind, modified, extension, created or random) and direction, view, preview column, and whether to reopen the last session or start in Home
- Optional per-folder view memory (settings): a folder opens in the view last chosen there
- Remembers hidden files, sort order, the sidebar, the preview column, the view mode and your open tabs
- Rename several items at once (`r` with a selection): find and replace, or a new name with `{n}`
  numbering, live preview with clashes in red, one undo for the whole batch
- New file (`N`), duplicate (`Ctrl+D`) and paste as symbolic link (`Ctrl+Shift+V`)
- Copy, cut, paste, rename, new folder and move to Trash, all with undo and redo; copy and paste work
  with other file managers and apps through the system clipboard (Wayland)
- Copies show live progress in the sidebar and can be cancelled; later operations queue; pasting over
  existing names asks to keep both, merge folders, skip or replace (replaced items go to the
  Trash), also by key (`Enter`, `m`, `s`, `r`); a move across disks checks the copied size before
  deleting the source
- Quick Look for images, folders, source code with syntax colour, rendered Markdown and the first
  pages of a PDF (`pdftoppm`, from poppler; `j` `k` turn pages) (`Space`)
- Archives (zip, tar.gz/xz/zst/bz2, 7z, rar, iso, cpio, jar): Quick Look and the preview column
  list what is inside, `e` extracts next to the archive into a folder named after it (one top-level
  folder is not nested twice); `c` compresses the selection to `.zip` (`.tar.gz` from the menu),
  undoable; `B` opens an unpacked copy in a new tab (kept in `~/.cache/files/archives`, changes do
  not go back into the archive). Uses `bsdtar` from libarchive, which refuses `..` and absolute paths
- Helpers that read untrusted files (`pdftoppm`, `pdfinfo`, `bsdtar`) run inside bubblewrap when
  `bwrap` is installed: no network, empty environment, read-only `/usr`, read access to the one
  input file and write access only to their output folder. Without `bwrap` they run directly
- Video and audio previews show duration and codecs (`ffprobe`), videos also a frame at 1 s
  (`ffmpeg`); fonts show a rendered specimen (ImageMagick). All three run in the sandbox
- Photos list camera, lens, date taken, exposure, aperture, ISO and focal length (`exiv2`); docx,
  odt, pptx and odp files preview their text, read from the document XML via `bsdtar`
- Preview column beside the list that follows the cursor (`F3`)
- Columns view (`Ctrl+2`): the parent folder on the left, the open folder in the middle, the
  folder under the cursor on the right; click a side column to move there
- Overview page (sidebar): disk usage per device, your home folder by file type, and recent files
  from `recently-used.xbel` (click one to open its folder with the file selected)
- More windows (`Ctrl+N`). Drag a tab out of the window, or press `Ctrl+Alt+N`, to move it into
  a new window with its history. Settings changed in one window apply to all of them, and every
  open window comes back with its tabs at the next start
- Dual pane (`Ctrl+4`): two folders side by side, `Tab` moves between them, `F6` copies the
  selection to the other side and `Shift+F6` moves it; drag the gap between them to resize
  (20–80 %, double-click resets, remembered across launches)
- Grid view with image thumbnails (`Ctrl+3`, back to the list with `Ctrl+1`), cached in the shared
  freedesktop thumbnail cache (`~/.cache/thumbnails`) that other file managers also use
- Rubber-band selection in the list and grid: drag from empty space; `Ctrl` or `Shift` adds to the
  current selection
- Drag and drop: drag rows or tiles onto a folder, the open folder or a sidebar place; same disk
  moves, another disk copies, `Ctrl` while dropping always copies. Files dragged out of the window
  go to other apps, and files dropped in from other apps are copied or moved the same way
- List columns: Size, Kind, Modified and Created can each be shown or hidden in settings and
  resized by dragging the left edge of their header; a narrow pane drops Kind and Created
- Extension badges coloured from a hash of the extension, so every type gets a stable colour
- Opening files with your default applications (`xdg-open`), or with any other app that lists the
  file type (`o`, default app first; terminal apps open in your terminal)
- A terminal in the current folder (`S`): `$TERMINAL` when set, otherwise the first of
  `xdg-terminal-exec`, kitty, foot, Alacritty, Ghostty, WezTerm, GNOME Terminal, Konsole, Xfce
  Terminal, Console or xterm found on your `PATH`

Languages: English and French, chosen from `LC_ALL` / `LC_MESSAGES` / `LANG` (a region such as
`fr_FR` falls back to `fr`). `python3 scripts/check_locales.py` fails when a locale misses or adds
keys compared with `locales/en.yml`.

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

Only one instance runs: launching `files` again opens the path as a new tab in the focused window. While `files` is
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

Two presets, picked under Keys in the settings panel and applied at once. **Vim** (the default)
is the table below. **Classic** drops every plain-letter key so typing a name jumps to it
(the typed letters reset after a second), and moves the letter commands to `F1` (this sheet),
`Shift+F10` / Menu key (context menu), `Alt+Enter` (properties), `Ctrl+Space` (mark), `Ctrl+I`
(invert), `Alt+S` (folder sizes), `Ctrl+Shift+F` (filter), `Alt+1`–`9` (tabs),
`Alt+PgDn` / `Alt+PgUp` (PDF pages in the preview column) and `Ctrl+Alt+1`–`7` / `Ctrl+Alt+0`
(tags). Ctrl, F-key, arrow and Space keys are the same in both. The keymap sheet shows the keys of
the active preset.

| Keys | Action |
| --- | --- |
| `j` `k` / `↓` `↑` | Move the cursor |
| `gg` / `G` | First / last row |
| `Enter` `l` `→` | Open |
| `o` | Open with another app |
| `i` | Properties: kind, location, size, dates, owner, permissions (`1`–`9` toggle, undoable) |
| `S` | Terminal in this folder |
| `e` | Extract an archive here |
| `h` `←` `Backspace` | Parent folder |
| `Alt+←` / `Alt+→` | Back / forward |
| `t` / `w` | New / close tab |
| `1`–`9`, `[` `]` | Go to a tab |
| `{` `}` / `Ctrl+Shift+PgUp` `PgDn` | Move the tab left / right (or drag it) |
| Mouse back / forward buttons | Back / forward |
| `/` / `Esc` | Filter this folder / clear the filter |
| `f` / `Ctrl+F` | Search names below this folder |
| `T` then `1`–`7` / `T` `0` | Toggle a colour tag / clear colour tags |
| `:` / `Ctrl+L` | Type a path |
| `v` / `Ctrl+A` / `Shift+↓↑` | Select / select all / extend |
| `Shift+→` / `Shift+←` | Expand a folder in place / collapse it (list view) |
| `V` | Invert the selection |
| `s` | Measure folder sizes (marked folders, or every folder here; stays on one disk) |
| `y` `x` `p` | Copy / cut / paste |
| `r` `F2` | Rename |
| `dd` `Delete` | Move to Trash |
| `Ctrl+Shift+N` | New folder |
| `z` / `Ctrl+Shift+Z` | Undo / redo |
| `Space` | Quick Look: `h` `l` / `←` `→` previous / next file, `j` `k` / `↓` `↑` scroll or turn PDF pages, `+` `−` zoom |
| `Ctrl+N` / `Ctrl+Alt+N` | New window on the current folder / move the current tab to a new window |
| `Ctrl+K` | Connect to a server (path bar prefilled with `sftp://`) |
| `F3` | Show or hide the preview column |
| `Shift+J` `Shift+K` | Next / previous page of a PDF in the preview column |
| `Ctrl+1` `Ctrl+2` `Ctrl+3` `Ctrl+4` | List, columns, grid, dual pane |
| `Tab` / `F6` / `Shift+F6` | Dual pane: other side / copy there / move there |
| `m` / right click | Context menu |
| `Ctrl+0` | Move between the sidebar and the file list; in the sidebar `j` `k` move, `Enter` opens a place, drive or server, or searches a tag, `x` ejects a drive, disconnects or forgets a server, `e` edits a saved server, `Esc` goes back |
| `,` / `Ctrl+,` | Settings |
| `Ctrl+Shift+C` | Copy path |
| `?` | Keyboard shortcuts |
| `Ctrl+H` | Show hidden files |
| `F5` / `Ctrl+R` | Reload and report how many rows were added, removed or changed (rarely needed, folders update live) |
| `Ctrl+B` | Collapse the sidebar |

## Licence

GPL-3.0-or-later. See [LICENSE](LICENSE).

The bundled [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) font is licensed under the
SIL Open Font License 1.1, see [assets/fonts/OFL.txt](assets/fonts/OFL.txt).
