# 🖼️ Belilo 🧼

![Stars](https://img.shields.io/github/stars/Inc44/belilo?style=social)
![Forks](https://img.shields.io/github/forks/Inc44/belilo?style=social)
![Watchers](https://img.shields.io/github/watchers/Inc44/belilo?style=social)
![Repo Size](https://img.shields.io/github/repo-size/Inc44/belilo)
![Language Count](https://img.shields.io/github/languages/count/Inc44/belilo)
![Top Language](https://img.shields.io/github/languages/top/Inc44/belilo)
[![Issues](https://img.shields.io/github/issues/Inc44/belilo)](https://github.com/Inc44/belilo/issues?q=is%3Aopen+is%3Aissue)
![Last Commit](https://img.shields.io/github/last-commit/Inc44/belilo?color=red)
[![Release](https://img.shields.io/github/release/Inc44/belilo.svg)](https://github.com/Inc44/belilo/releases)
[![Sponsor](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Inc44)

Belilo, which translates to 'whitewasher' in Russian, is a useful tool created with ❤️ using Rust. It quickly whitens images, providing a clean, uniform appearance. It's fast, efficient, and precise.

## ⚙️ Features

- 🚀 Fast image whitening powered by [Rayon](https://crates.io/crates/rayon).
- 🖼️ Supports many image formats (BMP, GIF, JPEG, PNG, QOI, TGA, and WEBP).
- 🗂️ Batch process multiple images at once.
- 📁 Organized output to keep trimmed images together.
- 🛠️ Easy-to-use command-line interface.

## ⚠️ Disclaimers

Input images with zero width or height (0x0) are skipped entirely, as according to image standards supported by this tool, they are not valid images by definition.

## 🚀 Installation from crates.io

```bash
cargo install belilo
```

## 🛠️ Build from Source

```bash
git clone https://github.com/Inc44/belilo.git
cd belilo
cargo build --release
```

## 📦 Publish

```bash
cargo publish
```

## 📖 Usage Example

```bash
cargo run --release <input_paths>... [options]
```

Or

```bash
belilo <input_paths>... [options]
```

## 🎨 Command-Line Arguments

| Argument                  | Description                                            |
|---------------------------|--------------------------------------------------------|
| `<input_paths>`           | Paths to the input images or directories (required)    |
| `-o, --override`          | Override the input image instead of creating a new one |
| `-k, --keep`              | Keep modification time                                 |
| `-t, --threshold <value>` | Threshold for whitening (0-255) (default: 220)         |

## 🐛 Bugs

Not yet found.

## ⛔ Known Limitations

Not yet known.

## 🙏 Thanks

Creators of:

- [Rust](https://www.rust-lang.org)
- [clap](https://github.com/clap-rs/clap)
- [filetime](https://github.com/alexcrichton/filetime)
- [Image](https://github.com/image-rs/image)
- [Rayon](https://github.com/rayon-rs/rayon)
- [walkdir](https://github.com/BurntSushi/walkdir)

## 🤝 Contribution

Contributions, suggestions, and new ideas are heartily welcomed. If you're considering significant modifications, please initiate an issue for discussion before submitting a pull request.

## 📜 License

[![MIT](https://img.shields.io/badge/License-MIT-lightgrey.svg)](https://opensource.org/licenses/MIT)

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 💖 Support

[![BuyMeACoffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/xamituchido)
[![Ko-Fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/inc44)
[![Patreon](https://img.shields.io/badge/Patreon-F96854?style=for-the-badge&logo=patreon&logoColor=white)](https://www.patreon.com/Inc44)