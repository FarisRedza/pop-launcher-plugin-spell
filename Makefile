prefix = $(HOME)/.local/share

all: src/spell

install: src/spell
	install -D src/* \
		-t $(DESTDIR)$(prefix)/pop-launcher/plugins/spell

clean:
	-rm -rf .venv

distclean: clean

uninstall:
	-rm -rf $(DESTDIR)$(prefix)/pop-launcher/plugins/spell

.PHONY: all install clean distclean uninstall
