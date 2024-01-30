# FITS viewer without installing

[![image](https://img.shields.io/badge/demo-rustfits-green)](https://kazewong.github.io/rustfits-web/)


FITS (Flexible Image Transport System) is a standard format for astronomical data. It is designed to store images, cubes, spectra, tables and other data, together with descriptive metadata. While there are many tools to view FITS files, most of them requires the user to install them and manage its dependencies. Imagine you are in a dire situation in a party which you need impress a friend by showing him/her a FITS file, but you don't have any FITS viewer installed on your phone, that's too bad.

This is why I created this project. It is a web-based FITS viewer which can run on the your web browser using your local resource and without a server. This means you can view FITS files on your computer, as well as your phone, tablet, or even your smart watch if you dare to dream. It also means there is no server will go down and so this is a very reliable way to view FITS files. You can still install and use it locally, but in case you have access to internet and absolutely don't want to set up anything, here is the link to the full fledge version of this project: 

## Features

- [x] Load fits file
- [x] Display header
- [ ] Display image
- [ ] Display table
- [ ] Manipulate image

## Serving locally

To build the website locally, you need to install three things and their dependencies:
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Trunk](https://trunkrs.dev/)
3. [Tailwindcss](https://tailwindcss.com/)

I modify the part of the original README.md from the trunk template for your reference. In addition, there is also instruction for installing tailwindcss.

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

The last thing we need to serve this webapp is tailwindcss.

```bash
curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss
chmod +x tailwindcss
PATH=$PATH:$(pwd)
```

If you see an error from the building pipeline about tailwindcss, that means you might not have tailwindcss installed properly. Make sure the executable is in your PATH.

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.
