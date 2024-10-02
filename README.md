# stegjs

[![npm](https://img.shields.io/npm/v/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs) [![npm](https://img.shields.io/npm/dt/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs) ![Steg.js Test workflow](https://github.com/andmev/stegjs/actions/workflows/test.yml/badge.svg)
[![npm](https://img.shields.io/npm/l/stegjs.svg?maxAge=1)](https://www.npmjs.com/package/stegjs)

> A Node.js module and command-line utility for performing steganographic encoding in PNG images. This application enables the secure transmission of secret messages, passwords, or other critical information by embedding data within images.

## Installation

To install globally you should enter in terminal window the following command:

```sh
$ npm i -g stegjs
```

To install locally you should enter in terminal window the following command:

```sh
$ npm i stegjs
```

## Help

### Usage as Node.js module

```js
const steg = require('stegjs')

// Encode message
const response = steg.encode('img.png', 'my_secret_pass', '1x1', './secrets/go.png')
console.log(response) // -> { message: 'my_secret_pass', pattern: '1x1', output: './secrets/go.png' }

// Decode message
const response = steg.decode('./secrets/go.png')
console.log(response) // -> { message: 'my_secret_pass', pattern: '1x1' }
```

### Usage as CLI

```sh
$ stegjs --help

  Usage: stegjs <image or url>.png [mode] [message] [step] [output]

  Encrypt you message to PNG image.

  Options:

    -h, --help     output usage information
    -V, --version  output the version number
    -e, --encode   Change program mode to encode file
    -d, --decode   Change program mode to decode file
```

Syntax information:

```
[mode]		one of the two modes
[message]	the message you want to encrypt
[step]		pattern of the distribution message bits in the alpha channel of the image
[output]	path where to save the image with the encoded message
```

## Examples

The program works in two modes, encrypt and decrypt messages.

#### Encode

To encrypt a message use one of the following commands:

```sh
$ npx stegjs img.png -e "Meeting tonight at midnight under the light." 5x5
$ npx stegjs https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png -e "my_secret_pass" 1x1 ./secrets/go.png
$ npx stegjs nyan.png -e "🐱" 2x1
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
$ npx stegjs out.png -d
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
