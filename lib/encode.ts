'use strict';

import { PNG } from 'pngjs';
import { red, yellow } from 'chalk';
import { createReadStream, createWriteStream } from 'fs';

import { byURI, byPath } from './getFile.js';
import { stringToBits } from './converters.js';
import { isRightStep, isPNG, isURI } from './checker.js';


/** The function for encoded message. */
const encodeImage = (img, msg, step, out) => {

    // Create the stream and start reading file asynchronously
    createReadStream(img)
        .pipe(new PNG({
            filterType: 4       // Throw to stream a new PNG file (that will be output)
        }))
        .on('parsed', function () {

            // Check pattern for correctness of input
            // Return two variables: step by width and step by height
            const [widthValue, heightValue] = isRightStep(step);

            // Get the length of the message.
            const textLength = msg.length;

            // Create a string with meta-information:
            // - Message length
            // - Step by width
            // - Step by height
            const meta = `${textLength}|${widthValue}|${heightValue}|`;

            // Encode meta-information to bits
            const arr = stringToBits(meta.toString());

            // Encode the message to bits
            const bits = stringToBits(msg);

            // Variables for iterating through bits array of meta information and message
            let index1 = 0;
            let index2 = 0;

            // Iterate through each pixel of the image height and width
            for (let y = 0; y < this.height; y++) {
                for (let x = 0; x < this.width; x++) {

                    // Get index
                    let idx = (this.width * y + x) << 2;

                    // Byte 254 has the form 11111110
                    // Conjunction with 254 saves all bits in the alpha channel in place
                    // but instead last writes 0.

                    // Disjunction with 1 also keeps all bits in place
                    // but last bit replace on 1.

                    // This is approach to write message in the image,
                    // which will change the bytes for a specific channel in the least significant bit.
                    // Read more: https://en.wikipedia.org/wiki/Least_significant_bit
                    if (arr[index2] === false) {
                        this.data[idx] &= 254
                    } else {
                        this.data[idx] |= 1
                    }
                    index2 += 1;

                    // Here we just write message in image
                    // but we write them on a given pattern.
                    if (y % heightValue === 0) {
                        if (x % widthValue === 0) {

                            if (bits[index1] === false) {
                                this.data[idx + 3] &= 254
                            } else {
                                this.data[idx + 3] |= 1
                            }
                            index1 += 1;
                        }
                    }

                    // If the image dimensions do not allow to write text with a given pattern,
                    // then throw error to user entered a lower step or length of message.
                    if (y === this.height - 1 && x === this.width - 1 && bits.length > index1) {
                        console.error(red(`
                        Error: A very long message or a wide step! This amount of text does not fit in this picture.
                        Please reduce step, length of the text or use image with higher resolution.`
                        ));
                        process.exit(1)
                    }
                }
            }

            // Save the image and then send the message to the console
            // what and where saved.
            this.pack().pipe(createWriteStream(out)).on('close', () => {
                console.log(`${out} has been encoded\nmessage: ${yellow(msg)}\npattern: ${step}`);
                process.exit(0)
            });
        });
};

/**
 * Function to encoding message and takes following parameters:
 * @param {string} img - path to image file or the URI.
 * @param {string} msg - message that user wishes to encode in the image.
 *     Supports all ASCII codes and Emoji.
 * @param {string} step - step of bits with message will be written to bits for
 *     the alpha channel of image.
 * @param {string} out - path to output file.
 */
export const encode = (img, msg, step, out) => {

    // Check that input file was in PNG format.
    if (isPNG(img)) {

        // Check if we come URI
        if (isURI(img)) {

            // Download image and return path to the file.
            byURI(img, (err, file) => {

                // Check for error (e.g. no permissions to read file)
                if (err) {
                    throw new URIError(err);
                } else {

                    // If all OK, send the path to file, message, step and path to output file
                    // to decoding function.
                    encodeImage(file, msg, step, out)
                }
            })
        } else {

            // If img parameter is not URI, then try to access it from the hard drive.
            byPath(img, (err, file) => {

                // Again check for error.
                if (err) {
                    throw new Error(err);
                } else {

                    // If all OK, send the path to file, message, step and path to output file
                    // to decoding function.
                    encodeImage(file, msg, step, out)
                }
            })
        }
    } else {

        // If user gave the file is not in PNG format, or rather not of type PNG,
        // then we send error.
        throw new TypeError('Only *.png images are supported.');
    }
};
