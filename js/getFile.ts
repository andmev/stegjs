import { createWriteStream } from 'fs';
import fetch from 'node-fetch';

import { hasAccess } from './checker';

/**
 * Function checks the readability of the file and sends the file path.
 */
export const byPath = async (imgPath: string): Promise<string | Error> => {
  try {
    return await hasAccess(imgPath);
  } catch (e) {
    return e as Error;
  }
};

/**
 * Function takes a url, downloads the file and sends the full path to it.
 */
export const byURI = async (uri: string): Promise<string> => {
  return new Promise((resolve, reject) => {
    const filename = uri.substring(uri.lastIndexOf('/') + 1);
    fetch(uri)
      .then((res) => {
        const destination = createWriteStream(filename);
        return res.body?.pipe(destination);
      })
      .then((stream) => {
        stream?.on('error', (err) => reject(err));
        stream?.on(
          'response',
          (res) =>
            res.statusCode >= 400 &&
            reject('Something wrong with URL or server'),
        );
        stream?.on('finish', () => resolve(`${process.cwd()}/${filename}`));
      });
  });
};
