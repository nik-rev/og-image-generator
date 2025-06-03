# og-image-generator

This project lets you easily generate beautiful Open Graph images for your static site using **HTML and CSS**!

It is distributed as **a single binary** and uses [blitz](https://github.com/DioxusLabs/blitz) to render HTML and CSS to an image, making it significantly more lightweight than something like a Headless Chromium wrapper.

It contains 2 subcommands:

- `cargo-image-generator single` generates a single OpenGraph image
- `cargo-image-generator all` generates OpenGraph images for each HTML file in a directory (recursive). In most cases, this what you want.

More information about them is found in the `--help` menu.

## Examples

Examples of generated open graph images

The [exact templating syntax](https://keats.github.io/tera/) is described in the Tera documentation.

### Carbon

Source file: [carbon.html](./examples/carbon.html)

```sh
og-image-generator single examples/carbon.html title="Announcing og-image-generator" description="A command-line tool that generates OpenGraph images from HTML and CSS" --font examples/Literata.ttf
```

Output:

<img src="examples/carbon.png" alt="carbon"></img>

### Party Time

Source file: [partytime.html](./examples/partytime.html)

```sh
og-image-generator single examples/partytime.html title="Announcing og-image-generator" description="A command-line tool that generates OpenGraph images from HTML and CSS" --font examples/HappyMonkey.ttf
```

Output:

<img src="examples/partytime.png" alt="partytime"></img>

## Generating OpenGraph images for a whole website, in a single command

The following command:

```sh
og-image-generator all
```

Allows you to generate an OpenGraph image for each HTML file in a subdirectory.

Instead of passing variables in the command line like so (for `og-image-generator single`):

```sh
title="Announcing og-image-generator" description="A command-line tool that generates OpenGraph images from HTML and CSS"
```

They have to be available as `<meta>` tags in each file, as following:

```html
<meta
  property="og-image-generator"
  key="title"
  value="Announcing og-image-generator"
/>
<meta
  property="og-image-generator"
  key="description"
  value="A command-line tool that generates OpenGraph images from HTML and CSS"
/>
```

You can put anything you want for the `key` and `value` attributes.

`og-image-generator` will generate OpenGraph images at the same path as the HTML file itself, plus `/og.png` at the end.

So, for a url like `/example.com/blog/new-blog-post`, an OpenGraph image will be available at `/example.com/blog/new-blog-post/og.png`.

## Installation

### Homebrew

```sh
brew install nik-rev/tap/og-image-generator
```

### PowerShell

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/nik-rev/og-image-generator/releases/latest/download/og-image-generator-installer.ps1 | iex"
```

### Shell

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/nik-rev/og-image-generator/releases/latest/download/og-image-generator-installer.sh | sh
```

### Nix

We have a `flake.nix`, so you can build from source with `cargo build`
