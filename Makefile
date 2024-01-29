PLUGIN_NAME = spell
PLUGIN_VERSION = 0.1

LOCAL_PLUGIN_DIR = $(HOME)/.local/share/pop-launcher/plugins

PKG_DIR = usr/lib/pop-launcher/plugins/$(PLUGIN_NAME)
PKG_NAME = pop-launcher-plugin-$(PLUGIN_NAME)_$(PLUGIN_VERSION)-1_all.deb

install:
	mkdir -p $(LOCAL_PLUGIN_DIR)/$(PLUGIN_NAME)
	cp -r src/* $(LOCAL_PLUGIN_DIR)/$(PLUGIN_NAME)
	chmod +x $(LOCAL_PLUGIN_DIR)/$(PLUGIN_NAME)

build_deb_package:
	mkdir -p PKG_SOURCE/$(PKG_DIR)
	cp -r debian PKG_SOURCE/DEBIAN
	cp -r src/* PKG_SOURCE/$(PKG_DIR)
	dpkg-deb --root-owner-group --build PKG_SOURCE $(PKG_NAME)
	rm -r PKG_SOURCE