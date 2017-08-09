'use strict';

const chalk = require('chalk')
    , check = require('./checker.js')
    , conv = require('./converters.js')
    , fs = require('fs')
    , get = require('./getFile.js')
    , PNG = require('pngjs').PNG;


/** Function decode messages. */
function decodeImage(img) {
    fs.createReadStream(img)
        .pipe(new PNG({
            filterType: 4
        }))
        .on('parsed', function () {

            // Keep the length and height of the image
            const width = this.width;
            const height = this.height;

            // Create variables for bits of meta information and message
            let stegotext = [];
            let index = 0;

            // First get meta-information:
            // - Text length
            // - step by width
            // - step by height
            for (let y = 0; y < height; y++) {
                for (let x = 0; x < width; x++) {
                    let idx = (width * y + x) << 2;
                    stegotext[index] = (this.data[idx] & 1) ? 1 : 0;
                    index += 1
                }
            }

            // Parse the data and return them in variables
            const [stringLength, widthStep, heightStep] = conv.metaToObj(conv.bitsToString(stegotext));

            index = 0;

            // Get message from the image
            for (let y = 0; y < height; y++) {
                for (let x = 0; x < width; x++) {
                    let idx = (width * y + x) << 2;

                    if (y % heightStep == 0) {
                        if (x % widthStep == 0) {
                            stegotext[index] = (this.data[idx + 3] & 1) ? 1 : 0;
                            index += 1;
                        }
                    }
                }
            }

            // Parse message
            let data = conv.bitsToString(stegotext).split('');

            // Cut by needed length
            data.splice(stringLength, stegotext.length - stringLength);

            // Print a message to user and where it was saved.
            console.log(`${img} was decoded!\nmessage: ${chalk.yellow(data.join(''))}\npattern: ${widthStep}x${heightStep}`);
            process.exit(0)
        })
}

/**
 * Function for decode image
 * @param {string} img - path to the image that contains encrypted text.
 */
module.exports = function (img) {
    if (check.isPNG(img)) {
        if (check.isURI(img)) {
            // Download file and return the path to the downloaded file.
            get.byURI(img, (err, file) => {

                // Check for error (e.g. no permissions to read file)
                if (err) {
                    console.error(chalk.red(err));
                    process.exit(1)
                } else {

                    // If all OK, send the path to file to decoding function.
                    decodeImage(file)
                }
            });
        } else {
            get.byPath(img, (err, file) => {
                if (err) {
                    console.error(chalk.red(err));
                    process.exit(1)
                } else {
                    decodeImage(file)
                }
            })
        }
    } else {
        console.error(chalk.red('Only *.png images supported.'));
        process.exit(1)
    }
};
