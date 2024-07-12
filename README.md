# Unicode Art
### The better ascii art

# Installation

### Using precompiled binaries
Download the precompiled binary for your platform from the [releases page](https://github.com/gre-v-el/uniart/releases) and place it in your path.

### From crates.io
You can install `uniart` directly from [crates.io](https://crates.io/crates/uniart) using cargo:
```sh
cargo install uniart
```

### From source
Alternatively, you can clone the repository and install it manually:
```sh
git clone https://github.com/gre-v-el/uniart
cd uniart
cargo install --path .
```
or
```sh
git clone https://github.com/gre-v-el/uniart
cd uniart
cargo build --release
cp target/release/uniart /usr/local/bin
```

# Examples
### Images
```sh
uniart mandelbrot.png
uniart mandelbrot.png -m luminance -tbfq
uniart mandelbrot.png -tbf -m double-pixels
uniart mandelbrot.png -tbf -m braille
```
![Screenshots of mandelbrot renderings](/examples/mandelbrots.png)

Each image is a screenshot of a text terminal.

### Animations
```sh
uniart mandelbrot.gif
uniart mandelbrot.gif -m luminance -tbf
uniart mandelbrot.gif -tbf -m double-pixels
uniart mandelbrot.gif -tbf -m braille
```
![Recordings of mandelbrot zoom animations](/examples/mandelbrots.gif)

These are real time recordings of the terminal. All gifs are rendered in the same resolution in characters.

# Features
* Convert images and gifs to characters on the terminal in one of the following modes:
  - `luminance`: Uses the luminance of the pixel to determine the character.
  - `braille`: Uses braille characters to represent the image.
  - `pixels`: Uses one color per character to represent the image.
  - `double-pixels`: As above, but uses two pixels per character.
  - `edges`: Uses edge detection to make edges of objects more visible.
  - `shapes`: Selects characters that have a similar shape to the represented area of the image.
* Uses given width, or adjusts to the size of the terminal window.
* Can take a custom palette of characters to use.
* Can display images in color. (256 ansi escape sequences color codes, or 24bit true color in some terminals)
* Can be also used in light themed terminals with brightness inversion.

# Usage
```
uniart [OPTIONS] <IMAGE>

<IMAGE>  Path to the image file

Output customization:
  -m, --mode <MODE>        Mode. (one of: luminance, pixels, double-pixels, braille, edges, shapes) [default: shapes]
  -w, --width <WIDTH>      Sets the width of the output. If set to 0 it will fill the terminal window [default: 0]
  -a, --aspect <ASPECT>    Sets the aspect ratio of the terminal font [default: 2]
  -p, --palette <PALETTE>  Sets the character palette to use. Works in shapes mode.

Color options:
  -c, --colors      Outputs the image in color.
  -t, --truecolor   Uses truecolor escape sequences. (only works in some terminals)
  -b, --background  Sets the background color to black.

Image manipulation:
  -i, --invert   Inverts the image brightness. (useful in white-background terminals)
  -q, --quality  Switch the quality of output. (only works for luminance, edges and shapes modes)
  -f, --filter   Uses linear filter instead of nearest neighbor when scaling the image.
```