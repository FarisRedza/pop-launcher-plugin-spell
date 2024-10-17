prefix = $(HOME)/.local/share

all: src/main.rs

build: src/main.rs
	@if [ -f vendor.tar ]; then \
		echo "vendor.tar found, extracting and building offline..."; \
		tar -xf vendor.tar; \
		cargo build --release --offline; \
	else \
		echo "vendor.tar not found, building online..."; \
		cargo build --release; \
	fi

install: build
	install -D src/plugin.ron \
		-t $(DESTDIR)$(prefix)/pop-launcher/plugins/spell
	chmod -x $(DESTDIR)$(prefix)/pop-launcher/plugins/spell/plugin.ron

	install -D target/release/spell \
		-t $(DESTDIR)$(prefix)/pop-launcher/plugins/spell

vendor:
	mkdir -p .cargo
	cargo vendor | head -n -1 > .cargo/config.toml
	echo 'directory = "vendor"' >> .cargo/config.toml
	tar pcf vendor.tar vendor
	rm -rf vendor

clean:
	-rm -rf target vendor

distclean: clean

uninstall:
	-rm -rf $(DESTDIR)$(prefix)/pop-launcher/plugins/spell

.PHONY: all build install clean distclean uninstall
