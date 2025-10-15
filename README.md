# Image Combine

A simple and efficient command-line tool written in Rust that allows you to combine multiple input images into a single output image. It supports flexible layout options, making it easy to stitch images together horizontally or vertically. This tool is designed to be fast, lightweight, and easy to integrate into scripts or workflows.

## 🧩 Example Usage

```sh
# Combine images left to right
image-combine img1.jpg img2.png img3.webp -o out.png

# Stack images top to bottom with spacing and dark background
image-combine img/*.png -l v --spacing 8 --bg "#1e1e1e" -o collage.png

# Use a list file (remember to quote @file if using PowerShell)
image-combine @test.txt -o out.png

# Use `--help` to see all options
image-combine --help
```
