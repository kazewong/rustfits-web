# FITS viewer without installing

FITS (Flexible Image Transport System) is a standard format for astronomical data. It is designed to store images, cubes, spectra, tables and other data, together with descriptive metadata. While there are many tools to view FITS files, most of them requires the user to install them and manage its dependencies. Imagine you are in a dire situation in a party which you need impress a friend by showing him/her a FITS file, but you don't have any FITS viewer installed on your phone, that's too bad.

This is why I created this project. It is a web-based FITS viewer which can run on the your web browser using your local resource and without a server. This means you can view FITS files on your computer, as well as your phone, tablet, or even your smart watch if you dare to dream. It also means there is no server will go down and so this is a very reliable way to view FITS files. You can still install and use it locally, but in case you have access to internet and absolutely don't want to set up anything, here is the link to the full fledge version of this project: 

## Features

- [x] Load fits file
- [x] Display header
- [ ] Display image
- [ ] Display table
- [ ] Manipulate image

## Serving locally

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

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

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

There are a few things you have to adjust when adopting this template.

### Remove example code

The code in [src/main.rs](src/main.rs) specific to the example is limited to only the `view` method.
There is, however, a fair bit of Sass in [index.scss](index.scss) you can remove.

### Update metadata

Update the `name`, `version`, `description` and `repository` fields in the [Cargo.toml](Cargo.toml) file.
The [index.html](index.html) file also contains a `<title>` tag that needs updating.

Finally, you should update this very `README` file to be about your app.

### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: `` and `kazewong <kazewong.physics@gmail.com>`.

[trunk]: https://github.com/thedodd/trunk
