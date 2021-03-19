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
 */
export const byURI = imgURI => {
    return new Promise((resolve, reject) => {
        const filename = imgURI.substring(imgURI.lastIndexOf('/') + 1);
        fetch(imgURI)
            .then(res => {
                const destination = createWriteStream(filename);
                return res.body.pipe(destination)
            })
            .then((stream) => {
                stream.on('error', err => reject(err))
                stream.on('response', res => res.statusCode >= 400 && reject('Something wrong with URL or server'))
                stream.on('finish', () => resolve(`${process.cwd()}/${filename}`))
            })
    })
};
