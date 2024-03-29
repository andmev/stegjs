import { access } from 'fs/promises';
import { constants } from 'fs';
import { join } from 'path';
import { URL } from 'url';

/**
 * Checks that the file extension was PNG.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */

// @todo Create better checking for PNG, for example exif or a unique byte.
export const isPNG = (imgPath: string): boolean => !!imgPath.match(/.png$/i);

/**
 * Check the file path to the URI.
 * @param {string} imgPath
 * @returns {Array|{index: number, input: string}|*}
 */
export const isURI = (imgPath: string): boolean => {
  try {
    const url = new URL(imgPath);
    return url.protocol === 'http:' || url.protocol === 'https:';
  } catch (_) {
    return false;
  }
};

/**
 * Checks read access to the file.
 * @param {string} imgPath
 */
export const hasAccess = async (imgPath: string): Promise<string | Error> => {
  try {
    imgPath = join(process.cwd(), imgPath);
    await access(imgPath, constants.R_OK);
    return imgPath;
  } catch (e) {
    return e as Error;
  }
};

/**
 * Checks the syntax of the pattern. Returns an array of constant step length
 * and height.
 * @param step
 * @returns {Array}
 */
export const isRightStep = (step: string): [string, string] | SyntaxError => {
  const err = new SyntaxError('Wrong step input. Check help!');
  const pattern = step.split(/[xх]/gi);
  if (pattern.length === 2) {
    return pattern.map((item) => {
      if (isNaN(parseInt(item, 10))) {
        return err;
      }
      return item;
    }) as [string, string];
  } else {
    return err;
  }
};
