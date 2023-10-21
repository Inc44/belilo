# 🖼️ Belilo 🧼

## Introduction
Belilo, which translates to 'whitewasher' in Russian, is a useful tool created with ❤️ using Rust. It quickly whitens images, providing a clean, uniform appearance. It's fast, efficient, and precise.

## Features 🌟
- 🚀 Fast image whitening powered by [Rayon](https://crates.io/crates/rayon).
- 🖼️ Supports many image formats (JPEG, PNG, BMP, GIF, ICO, QOI, and more).
- 🗂️ Batch process multiple images at once.
- 📁 Organized output to keep trimmed images together.
- 🛠️ Easy-to-use command-line interface.

## How to Use 💼

If you are on Windows, simply download the .exe file. For all other operating systems, refer to the "Build the Project" section for compilation instructions.

## Build the Project 🚀

1. Clone the repository:
```bash
$ git clone https://github.com/Inc44/belilo.git
```
2. Go to the project directory:
```bash
$ cd belilo
```
3. Build and run the project:
```bash
$ cargo run --release <input-path>
```

## Contribution 🤝
We welcome contributions! For significant changes, please open an issue for discussion before making a pull request.

## License 📜
This software is licensed under the MIT Massachusetts Institute of Technology (MIT). For more details, refer to [LICENSE](LICENSE.md).