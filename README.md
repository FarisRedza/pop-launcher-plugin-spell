# pop-launcher-plugin-spell
A spell checking plugin for [pop-launcher](https://github.com/pop-os/launcher) using [aspell](https://github.com/GNUAspell/aspell). Searches for the closest matching spelling and copies it to your clipboard 

### Dependencies
[aspell](https://github.com/GNUAspell/aspell)\
This is preinstalled in Pop!_OS and most other Linux distributions
Debian/Ubuntu and derivatives - `sudo apt install aspell`\
Nix package - `nix-env -iA nixpkgs.aspell`

[xclip](https://github.com/astrand/xclip)\
Required for copying to the clipboard\
Debian/Ubuntu and derivatives - `sudo apt install xclip`\
Nix package - `nix-env -iA nixpkgs.xclip`

### Installation
For user installation, clone the repo and run `make install` inside the repo's folder. Make sure the dependencies are installed\
Uninstall with `make uninstall` when in the repo's folder

For system-wide installation, download and install the deb file

### Usage
In pop launcher type spell followed by your query to search for the closest matching spelling. Pressing enter copies the result into the clipboard.

### Todo
- [x] Replace hunspell with aspell
- [ ] Modify the GitHub action for creating a deb package to be closer to the method used for packaging for debian repos
