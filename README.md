# pop-launcher-plugin-spell
A spell checking plugin for [pop-launcher](https://github.com/pop-os/launcher) using [hunspell](https://github.com/hunspell/hunspell). Searches for the closest matching spelling and copies it to your clipboard 

### Dependencies
[hunspell](https://github.com/hunspell/hunspell)\
Debian/Ubuntu and derivatives - `sudo apt install hunspell`\
Nix package - `nix-env -iA nixpkgs.hunspell`

[xclip](https://github.com/astrand/xclip)\
Required for copying to the clipboard\
Debian/Ubuntu and derivatives - `sudo apt install xclip`\
Nix package - `nix-env -iA nixpkgs.xclip`

### Installation
For user installation, clone the repo and run `make install`. Make sure the dependencies are installed\
For system-wide installation, download and install the deb file

### Usage
In pop launcher type spell followed by your query to search for the closest matching spelling. Pressing enter copies the result into the clipboard.
