# pop-launcher-plugin-spell
A spell checking plugin for [pop-launcher](https://github.com/pop-os/launcher) using [aspell](https://github.com/GNUAspell/aspell). Searches for the closest matching spelling and copies it to your clipboard 

### Dependencies
[aspell](https://github.com/GNUAspell/aspell)\
This is preinstalled in Pop!_OS and most other Linux distributions\
Debian/Ubuntu and derivatives - `sudo apt install aspell`\
Nix package - `nix-env -iA nixpkgs.aspell`

### Installation
For user installation, clone the repo and run `make install` inside the repo's folder. This requires [cargo](https://doc.rust-lang.org/cargo/) to be installed\
Uninstall with `make uninstall` when in the repo's folder

For system-wide installation, download and install the deb file

### Usage
In pop launcher type spell followed by your query to search for the closest matching spelling. Pressing enter copies the result into the clipboard.
