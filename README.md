[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Project_license][license-shield]][license-url]

# Arch Update Checker
A lightweight Rust-based tool to monitor system updates on Arch Linux. Designed to be integrated with Waybar, it shows pending updates for Pacman and Flatpak.

## Screenshots

![Screenshot when pacman has updates](/screenshots/pacman.png)
![Screenshot when flatpak has updates](/screenshots/flatpak.png)

## Installation

### Prerequisites
- **Pacman** (with `checkupdates` from `pacman-contrib`) for checking system updates
- **Flatpak** for checking Flatpak updates
- **Cargo** with Rust 2024 support to build the project

### Installing and building the project
1. Clone this repository
```sh
git clone https://github.com/BloemGamer/arch_update_checker.git
cd arch_update_checker
```
2. Build the project
```sh
cargo build --release
```
3. Copy the binary to a folder in your PATH or use the abolute path in the config
```sh
sudo cp target/release/arch_update_checker /usr/local/bin/
```

### Adding to your config
4. Add a config to your waybar config
```waybar-config
"custom/updates": {
	"format": "{}",
	"return-type": "json",
	"exec": "/usr/local/bin/arch_update_checker --pacman --flatpak",
	"interval": 600,
	"tooltip": true,
	"on-click": "sh -c '\"$TERMINAL\" -e bash -c \"echo -e \\\"Command for updating:\\\"; /run/media/bloem/D/programmeren/arch_update_checker/arch_update_checker --pacman --flatpak --update-command; read -p \\\"Press Enter to close...\\\"\"'"
}
```
Update the `exec` path to where the final binary is on your machine.
Make sure `$TERMINAL` is set, or change the variable to your terminal of choosing.

## License
Arch Update Checker is released under the MIT License. See [LICENSE](LICENSE) for details.

## Contact
Feel free to reach out via [GitHub Discussions](https://github.com/BloemGamer/arch_update_checker/discussions) or [GitHub Issues](https://github.com/BloemGamer/arch_update_checker/issues).

[forks-shield]: https://img.shields.io/github/forks/BloemGamer/arch_update_checker.svg?style=flat
[forks-url]: https://github.com/BloemGamer/arch_update_checker/network/members
[stars-shield]: https://img.shields.io/github/stars/BloemGamer/arch_update_checker.svg?style=flat
[stars-url]: https://github.com/BloemGamer/arch_update_checker/stargazers
[issues-shield]: https://img.shields.io/github/issues/BloemGamer/arch_update_checker.svg?style=flat
[issues-url]: https://github.com/BloemGamer/arch_update_checker/issues
[license-shield]: https://img.shields.io/github/license/BloemGamer/arch_update_checker.svg?style=flat
[license-url]: https://github.com/BloemGamer/arch_update_checker/blob/main/LICENSE
