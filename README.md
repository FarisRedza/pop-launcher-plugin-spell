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

### Known issues
For a lot of cases where a word has a suffix, the plugin assumes the spelling is incorrect even when there is no spelling error, and will offer the same word without the suffix. This is an issue with hunspell. To solve this, hunspell will be replaced with aspell. aspell would be preferable anyway since it is pre-installed in Pop!_OS, and hunspell was only chosen as this plugin was inspired by an equivalent plugin for Albert launcher which used hunspell

### Todo
- [ ] Replace hunspell with aspell
- [ ] Include a Wayland compatible method of copying to clipboard as xclip seems to lack this functionality
- [ ] Modify the GitHub action for creating a deb package to be closer to the method used for packaging for debian repos
