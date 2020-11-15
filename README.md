# dominant-color 🔖
*Fast & minimal python module to compute the dominant color of an image, written in Rust.*

## Usage

Pre-compiled binaries are **available for Linux** using pip :
```
pip install dominantcolor
```

Alternatively, you can download wheel directly from [releases](https://github.com/baptiste0928/dominant-color/releases/latest), or build it yourself.

This module is **written in Rust**, so it's faster than a pure-python module. Usage is very intuitive :

```python
from dominantcolor import get_dominant_color

# Let's open an image as bytes object.
# This is just for the example, you can use any bytes object that
# correspond to a valid image format (https://crates.io/crates/image#supported-image-formats).
img = open("image.png", "rb").read()

# Now we call the function to compute the dominant color
color = get_dominant_color(img)

# The color is returned as an int, so we convert it to hex to make
# it more readable.
print(hex(color))
```

If image decoding failed, an `dominantcolor.DecodingError` exception is raised.

Internally, the module compute the HSL value of each pixel (max. 50k) and classifies each pixel using its hue.
The average of the biggest group of pixels is returned.

## Build-it yourself

Before you start, ensure [Rust is installed](https://www.rust-lang.org/tools/install) on your computer.

```
$ # Clone the repository
$ git clone https://github.com/baptiste0928/dominant-color.git & cd dominant-color
Cloning into 'dominant-color'...
...

$ # Install maturin (build tool)
$ pip install maturin
...

$ # Build python package
$ maturin build --release
...
📦 Built wheel for CPython 3.8 to ...


```

Python package (as wheel) can be found at `dominant-color/target/wheels/`.
It can be installed for any project with `pip install dominantcolor[...].whl`.

## Contributing

This is my first real project in Rust, I will be happy to receive contributions to improve it! 🙌
Feel free to open an issue or a PR if you want to contribute.