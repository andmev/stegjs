
# stegjs

[![npm](https://img.shields.io/npm/v/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs) [![npm](https://img.shields.io/npm/dt/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs) [![Build Status](https://travis-ci.org/andmev/stegjs.svg?branch=master)](https://travis-ci.org/andmev/stegjs) [![npm](https://img.shields.io/npm/l/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs)

> Command-line utility for steganography in PNG images. With this application you can send secret messages, passwords or other important information.

## Installation

To install globally you should enter in terminal window the following command:

```sh
$ npm i -g stegjs
```


## Help

```sh
$ stegjs --help

  Usage: stegjs <image or url>.png [mode] [message] [step] [output]

  Encrypt you message to PNG image.

  Options:

    -h, --help     output usage information
    -V, --version  output the version number
    -e, --encode   Change program mode to encode file.
    -d, --decode   Change program mode to decode file
```

Syntax information:

```
[mode]		one of the two modes;
[message]	the message you want to encrypt;
[step]		pattern of the distribution message bits in the alpha channel of the image;
[output]	path where to save the image with the encoded message.
```


## Examples

The program works in two modes, encrypt and decrypt messages.

#### Encode

To encrypt a message use one of the following commands:

```sh
$ stegjs img.png -e 'Meeting tonight at midnight under the light.' 5x5
$ stegjs https://google.com/img.png -e 'Xod(}bgwh2^j7>B8X' 1x1 ./secrets/go.png
$ stegjs nyan.png -e '🐱' 2x1
```

After that in the console, you will see the full path to the output image, message and pattern.

```sh
/Users/you/secrets/out.png has been encoded
message: 🐱
pattern: 2x1
```


#### Decode

To receive an encrypted message, specify the path to the image with the secret message and add the flag `-d`.


```sh
$ stegjs out.png -d
```

After that in the console, you will see the information contained in the encrypted image.

```sh
out.png was decoded
message: 🐱
pattern: 2x1
```


## License

[MIT][license] © [Andrey Medvedev][website]

[license]: http://showalicense.com/?fullname=Andrey%20Medvedev%20%3Ca.medvedev@me.com%3E&year=2016#license-mit
[website]: https://github.com/andmev
