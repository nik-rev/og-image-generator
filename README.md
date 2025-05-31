# og-image-generator

This project lets you easily generate beautiful Open Graph images for your blog using **HTML and CSS**!

It is distributed as **a single binary** and uses [blitz](https://github.com/DioxusLabs/blitz) to render HTML and CSS to an image, making it significantly more lightweight than something like a Headless Chromium wrapper.

## Examples

```sh
og-image-generator examples/carbon.html title="Announcing og-image-generator" description="A command-line tool that generates OpenGraph images from HTML and CSS" --font examples/Literata.ttf
```

<img src="examples/carbon.png" alt="carbon"></img>

```sh
og-image-generator examples/partytime.html title="Announcing og-image-generator" description="A command-line tool that generates OpenGraph images from HTML and CSS" --font examples/HappyMonkey.ttf
```

<img src="examples/partytime.png" alt="partytime"></img>
