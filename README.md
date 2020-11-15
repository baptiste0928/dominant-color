# dominant-color 🔖
*Fast & minimal python module to compute the dominant color of an image, written in Rust.*

[![](https://img.shields.io/pypi/v/dominantcolor)](https://pypi.org/project/dominantcolor/)
[![](https://img.shields.io/pypi/format/dominantcolor)](https://pypi.org/project/dominantcolor/)
[![](https://img.shields.io/pypi/l/dominantcolor)](https://github.com/baptiste0928/dominant-color/blob/main/LICENSE)


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

## Benchmarks

In order to know how fast this module is, I performed some tests on my computer (with an Intel Core i5 9300H).
The tests were performed with [timeit](https://docs.python.org/3/library/timeit.html) and the given value is **an average over 1000 executions**.

- [80x67 PNGA](https://pixabay.com/vectors/logo-bird-vector-swinging-design-1933884/) : `1.65 ms`
- [148x100 PNG](https://pixabay.com/illustrations/yoga-sunrise-silhouette-dawn-woman-5508336/) : `2.72 ms`
- [640x430 JPEG](https://unsplash.com/photos/93zqOgDn89U) : `24.74 ms`
- [2400x3600 JPEG](https://unsplash.com/photos/CyMNPopAFNY) : `123.66 ms`

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