# image-combine

A fast, lightweight CLI for combining multiple images into one output image.

It can place images side by side or stack them vertically, with optional spacing and a custom background color. It works well for quick collages, comparisons, sprite sheets, and scriptable image workflows.

## Features

- Combine multiple images into one file
- Horizontal or vertical layout
- Configurable spacing between images
- Custom background color with hex input
- Supports glob input like `img/*.png`
- Supports list files with `@file.txt`
- Simple CLI, easy to use in scripts

## Installation

### From source

```sh
cargo install --path .
```

### Build manually

```sh
cargo build --release
```

The compiled binary will be available at:

```sh
target/release/image-combine
```

## Usage

```sh
image-combine [OPTIONS] <IMAGES>...
```

## Examples

### Combine left to right

```sh
image-combine img1.jpg img2.png img3.webp -o out.png
```

### Stack top to bottom

```sh
image-combine img1.png img2.png img3.png -l v -o out.png
```

### Add spacing and background color

```sh
image-combine img/*.png -l v --spacing 8 --bg "#1e1e1e" -o collage.png
```

### Use a list file

```sh
image-combine @test.txt -o out.png
```

If you use PowerShell, quote the `@file` form:

```powershell
image-combine "@test.txt" -o out.png
```

## Arguments

### `<IMAGES>...`

One or more input images.

Inputs may be:

- direct file paths
- glob patterns such as `img/*.png`
- list files prefixed with `@`

List files should contain one input per line. Empty lines are ignored.

Example `images.txt`:

```txt
a.png
b.png
img/*.jpg
```

## Options

| Option | Description |
|---|---|
| `-o, --output <PATH>` | Output image path. Default: `output.png` |
| `-l, --layout <h|v>` | Layout direction: horizontal (`h`) or vertical (`v`) |
| `--spacing <PIXELS>` | Space between images in pixels |
| `-b, --bg <HEX>` | Background color in `#RRGGBB` or `#RRGGBBAA` format |
