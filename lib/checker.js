'use strict';

const fs = require('fs');
const path = require('path');


/**
 * Checks that the file extension was PNG.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */

// @todo Create better checking for PNG, for example exif or a unique byte.
exports.isPNG = function (imgPath) {
    return imgPath.match(/.png$/i);
};


/**
 * Check the file path to the URI.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */
exports.isURI = function (imgPath) {
    const url = require('url');
    return url.parse(imgPath).protocol;
};


/**
 * Checks read access to the file.
 * @param {string} imgPath
 * @param callback
 */
exports.hasAccess = function (imgPath, callback) {
    imgPath = path.join(process.cwd(), imgPath);
    fs.access(imgPath, fs.constants.R_OK, (err) => {
        callback(err);
    });
};


/**
 * Checks the syntax of the pattern. Returns an array of constant step length and height.
 * @param step
 * @returns {Array}
 */
exports.isRightStep = function (step) {
    const err = new SyntaxError('Wrong step input. Check help!');
    const divider = step.match(/x|х/gi)[0];
    const pattern = step.split(divider);
    if (pattern.length == 2) {
        return pattern.map(item => {
            if (isNaN(item)) throw err;
            return item
        })
    } else {
        throw err;
    }
};
