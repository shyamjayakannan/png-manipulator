<div align="center">
    <img src="./logo.png" width="400">
	<h1>Png Manipulator</h1>

Use this tool to encode secret messages in png files and send them to your friends. Then, decode the images that they send you! 
</div>
<br>

## Live Tool

You can use the live tool [`here`].

[`here`]: https://shyamjayakannan.github.io/png-manipulator
<br>

## About

The tool works by converting the png into a byte array and adds a chunk of a specific type, containing the message to it. Then, the bytes are re-converted into a png file. No difference can be spotted by a visual inspection of the two pngs, which helps conceal the fact that there is hidden data.

While decoding, the tool looks for the same chunk type that it uses for encoding messages in the byte array after converting the png.

A very helpful source to learn about png files can be [`found here`].

[`found here`]: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
<br>

## How It's made

The code is written partly in [`Rust`] and Vanilla Javascript. No frameworks were used.

The Rust code was converted to [`WebAssembly`], which provides an API for Javascript to use Rust functions in the browser.

The project uses the [`create-wasm-app`] template, which uses Webpack to bundle the Rust-generated WebAssembly.

[`Rust`]: https://www.rust-lang.org
[`WebAssembly`]: https://webassembly.org
[`create-wasm-app`]: https://github.com/rustwasm/create-wasm-app
<br>

## Dependencies

- [`crc`]: Rust implementation of [`CRCs`].

[`CRCs`]: https://en.wikipedia.org/wiki/Cyclic_redundancy_check
[`crc`]: https://github.com/mrhooray/crc-rs
<br>

## Contributing

Contributions are most welcome.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open [`a Pull Request`].

Don't forget to give the project a star! Thanks again!

[`a Pull Request`]: https://github.com/shyamjayakannan/png-manipulator/pulls
