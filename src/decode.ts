/* eslint-disable no-console */
import { red, yellow } from 'chalk';
import { createReadStream } from 'fs';
import { PNG } from 'pngjs';

import { bitsToString, metaToObj } from './converters.js';
import { byPath, byURI } from './getFile.js';
import { isPNG, isURI } from './checker.js';

type decodeImageType = (img: string) => Promise<void>;

/** Function decode messages. */
const decodeImage = (img: string) => {
  createReadStream(img)
    .pipe(
      new PNG({
        filterType: 4,
      }),
    )
    .on('parsed', function () {
      // Keep the length and height of the image
      const width = this.width;
      const height = this.height;

      // Create variables for bits of meta information and message
      const secretText = [];
      let index = 0;

      // First get meta-information:
      // - Text length
      // - step by width
      // - step by height
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const idx = (width * y + x) << 2;
          secretText[index] = this.data[idx] && 1 ? 1 : 0;
          index += 1;
        }
      }

      // Parse the data and return them in variables
      const [stringLength, widthStep, heightStep] = metaToObj(
        bitsToString(secretText),
      );

      index = 0;

      // Get message from the image
      for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
          const idx = (width * y + x) << 2;

          if (y % parseInt(heightStep, 10) === 0) {
            if (x % parseInt(widthStep, 10) === 0) {
              secretText[index] = this.data[idx + 3] && 1 ? 1 : 0;
              index += 1;
            }
          }
        }
      }

      // Parse message
      const data = bitsToString(secretText).split('');

      // Cut by needed length
      data.splice(
        parseInt(stringLength, 10),
        secretText.length - parseInt(stringLength, 10),
      );

      // Print a message to user and where it was saved.
      console.log(
        `${img} was decoded!\nmessage: ${yellow(
          data.join(''),
        )}\npattern: ${widthStep}x${heightStep}`,
      );
      process.exit(0);
    });
};

/**
 * Function for decode image
 * @param {string} img - path to the image that contains encrypted text.
 */
export const decode: decodeImageType = async (img) => {
  if (isPNG(img)) {
    try {
      if (isURI(img)) {
        // Download file and return the path to the downloaded file.
        const file = await byURI(img);

        // If all OK, send the path to file to decoding function.
        decodeImage(file);
      } else {
        const file = await byPath(img);
        decodeImage(file);
      }
    } catch (e) {
      console.error(red(e.message));
      process.exit(1);
    }
  } else {
    console.error(red('Only *.png images supported.'));
    process.exit(1);
  }
};
