PLUGIN_NAME = spell

PLUGIN_DIR = $(HOME)/.local/share/pop-launcher/plugins
PLUGIN = $(PLUGIN_DIR)/$(PLUGIN_NAME)
PACKAGE_DIR = usr/pop-launcher/plugins/${PLUGIN_NAME}

install:
	mkdir -p $(PLUGIN)
	cp plugin.ron $(PLUGIN)
	cp $(PLUGIN_NAME) $(PLUGIN)
	chmod +x $(PLUGIN)/$(PLUGIN_NAME)

deb_package:
	mkdir -p $(PACKAGE_DIR)
	cp plugin.ron $(PACKAGE_DIR)
	cp $(PLUGIN_NAME) $(PACKAGE_DIR)