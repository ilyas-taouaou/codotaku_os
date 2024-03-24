My first OS being made in a livestream
following this [guide](https://os.phil-opp.com/), checkout the livestream [here](https://youtu.be/6YMqsCJBxt8).

Compiling is as simple as:
```sh
cargo b
```
Running requires [qemu](https://www.qemu.org/download/) installed and in the path, and requires bootimage installed too.
to install bootimage, run:
```sh
cargo install bootimage
```
then you can easily run the compiled operating system on qemu directly:
```sh
cargo r
```