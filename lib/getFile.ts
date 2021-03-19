'use strict';

import { createWriteStream } from 'fs';
import * as fetch from 'node-fetch';

import { hasAccess } from './checker';

/**
 * Function checks the readability of the file and sends the file path.
 * @param {string} imgPath
 */
export const byPath = async (imgPath) => {
    try {
        return await hasAccess(imgPath);
    } catch (e) {
        return e;
    }
};

/**
 * Function takes a url, downloads the file and sends the full path to it.
 * @param {string} imgURI
 * @param callback
 */
export const byURI = (imgURI, callback) => {
    const filename = imgURI.substring(imgURI.lastIndexOf('/') + 1);
    fetch(imgURI)
        .then(res => {
            const destination = createWriteStream(filename);
            return res.body.pipe(destination)
        })
        .then((stream) => {
            stream.on('error', err => callback(err, null))
            stream.on('response', res => res.statusCode >= 400 ? callback('Something wrong with URL or server', null) : null)
            stream.on('finish', err => callback(err, `${process.cwd()}/${filename}`))
        })
};
