# pop-launcher-plugin-spell
A spell checking plugin for pop-launcher using hunspell. Searches for the closest matching spelling and copies it to your clipboard 

### Dependencies
[hunspell](https://github.com/hunspell/hunspell)\
Debian/Ubuntu and derivatives - `sudo apt install hunspell`\
Nix package - `nix-env -iA nixpkgs.hunspell`

[xclip](https://github.com/astrand/xclip)\
Required for copying to the clipboard\
Debian/Ubuntu and derivatives - `sudo apt install xclip`\
Nix package - `nix-env -iA nixpkgs.xclip`

### Installation
Clone the repo and run `make install`

### Usage
In pop launcher type spell followed by your query to search for the closest matching spelling. Pressing enter copies the result into the clipboard.
