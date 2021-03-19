'use strict';

import { constants } from 'fs';
import { access } from 'fs/promises';
import { join } from 'path';


/**
 * Checks that the file extension was PNG.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */

// @todo Create better checking for PNG, for example exif or a unique byte.
export const isPNG = imgPath => imgPath.match(/.png$/i);

/**
 * Check the file path to the URI.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */
export const isURI = imgPath => {
    try {
        const url = new URL(imgPath);
        return url.protocol === "http:" || url.protocol === "https:";
    } catch (_) {
        return false;
    }
};

/**
 * Checks read access to the file.
 * @param {string} imgPath
 */
export const hasAccess = async imgPath => {
    try {
        imgPath = join(process.cwd(), imgPath);
        await access(imgPath, constants.R_OK)
        return imgPath;
    } catch (e) {
        return e;
    }
};

/**
 * Checks the syntax of the pattern. Returns an array of constant step length
 * and height.
 * @param step
 * @returns {Array}
 */
export const isRightStep = step => {
    const err = new SyntaxError('Wrong step input. Check help!');
    const divider = step.match(/[xх]/gi)[0];
    const pattern = step.split(divider);
    if (pattern.length === 2) {
        return pattern.map(item => {
            if (isNaN(item)) throw err;
            return item
        })
    } else {
        throw err;
    }
};
