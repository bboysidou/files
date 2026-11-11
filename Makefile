APP := files
APP_ID := files_file_manager
VERSION := $(shell sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
ARCH := $(shell uname -m)

PREFIX ?= /usr/local
DESTDIR ?=
BINDIR := $(PREFIX)/bin
DATADIR := $(PREFIX)/share
ICONDIR := $(DATADIR)/icons/hicolor

CARGO ?= cargo
MAGICK ?= magick
RELEASE_BIN := target/release/$(APP)
LOGO := assets/logo/files_logo.png
ICONS := assets/logo/icons
ICON_SIZES := 16 24 32 48 64 128 256 512
DESKTOP := packaging/$(APP_ID).desktop
METAINFO := packaging/$(APP).metainfo.xml
METAINFO_ID := io.github.bboysidou.files

DIST := dist
BIN_NAME := $(APP)-$(VERSION)-$(ARCH)-linux
SRC_NAME := $(APP)-$(VERSION)

.PHONY: all build install uninstall icons dist dist-bin dist-src clean help

all: build

help:
	@echo "make                 build the release binary"
	@echo "make install         install under PREFIX (default /usr/local); use sudo for system paths"
	@echo "make uninstall       remove what install put in place"
	@echo "make dist            binary tarball, source tarball and SHA256SUMS in $(DIST)/"
	@echo "make icons           regenerate $(ICONS)/ from $(LOGO) (needs ImageMagick)"
	@echo "make clean           remove target/ and $(DIST)/"
	@echo "variables: PREFIX=$(PREFIX) DESTDIR=$(DESTDIR)"

build:
	$(CARGO) build --release --locked

$(RELEASE_BIN):
	@echo "$(RELEASE_BIN) is missing: run 'make' first (without sudo)" >&2
	@exit 1

install: $(RELEASE_BIN)
	install -Dm755 $(RELEASE_BIN) $(DESTDIR)$(BINDIR)/$(APP)
	install -Dm644 $(DESKTOP) $(DESTDIR)$(DATADIR)/applications/$(APP_ID).desktop
	install -Dm644 $(METAINFO) $(DESTDIR)$(DATADIR)/metainfo/$(METAINFO_ID).metainfo.xml
	for size in $(ICON_SIZES); do \
		install -Dm644 $(ICONS)/$$size.png $(DESTDIR)$(ICONDIR)/$${size}x$${size}/apps/$(APP_ID).png || exit 1; \
	done
	install -Dm644 LICENSE $(DESTDIR)$(DATADIR)/licenses/$(APP)/LICENSE
	install -Dm644 README.md $(DESTDIR)$(DATADIR)/doc/$(APP)/README.md
	@if [ -z "$(DESTDIR)" ]; then \
		gtk-update-icon-cache -qtf $(ICONDIR) 2>/dev/null || true; \
		update-desktop-database -q $(DATADIR)/applications 2>/dev/null || true; \
	fi

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/$(APP)
	rm -f $(DESTDIR)$(DATADIR)/applications/$(APP_ID).desktop
	rm -f $(DESTDIR)$(DATADIR)/metainfo/$(METAINFO_ID).metainfo.xml
	for size in $(ICON_SIZES); do \
		rm -f $(DESTDIR)$(ICONDIR)/$${size}x$${size}/apps/$(APP_ID).png; \
	done
	rm -rf $(DESTDIR)$(DATADIR)/licenses/$(APP) $(DESTDIR)$(DATADIR)/doc/$(APP)
	@if [ -z "$(DESTDIR)" ]; then \
		gtk-update-icon-cache -qtf $(ICONDIR) 2>/dev/null || true; \
		update-desktop-database -q $(DATADIR)/applications 2>/dev/null || true; \
	fi

icons:
	mkdir -p $(ICONS)
	for size in $(ICON_SIZES); do \
		$(MAGICK) $(LOGO) -filter Lanczos -resize $${size}x$${size} -strip PNG32:$(ICONS)/$$size.png || exit 1; \
	done

dist: dist-bin dist-src
	cd $(DIST) && sha256sum $(BIN_NAME).tar.gz $(SRC_NAME).tar.gz > SHA256SUMS

dist-bin: build
	rm -rf $(DIST)/$(BIN_NAME)
	$(MAKE) --no-print-directory install DESTDIR=$(CURDIR)/$(DIST)/$(BIN_NAME) PREFIX=
	tar -C $(DIST) -czf $(DIST)/$(BIN_NAME).tar.gz $(BIN_NAME)
	rm -rf $(DIST)/$(BIN_NAME)

dist-src:
	mkdir -p $(DIST)
	git archive --format=tar.gz --prefix=$(SRC_NAME)/ -o $(DIST)/$(SRC_NAME).tar.gz HEAD

clean:
	$(CARGO) clean
	rm -rf $(DIST)
