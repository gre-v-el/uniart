# Unicode Art
### The better ascii art

## Examples
#### Images
```
uniart mandelbrot.png
uniart mandelbrot.png -m luminance -tbfq
uniart mandelbrot.png -tbf -m double-pixels
uniart mandelbrot.png -tbf -m braille
```
![mandelbrot](./examples/mandelbrots.png)
Each image is a screenshot of a text terminal.

#### Animations
```
uniart mandelbrot.gif
uniart mandelbrot.gif -m luminance -tbf
uniart mandelbrot.gif -tbf -m double-pixels
uniart mandelbrot.gif -tbf -m braille
```
![mandelbrot](./examples/mandelbrots.gif)
These are real time recordings of the terminal.

## Features
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

