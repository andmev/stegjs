'use strict';

const fs = require('fs')
    , check = require('./checker')
    , fetch = require('node-fetch');


/**
 * Function checks the readability of the file and sends the file path.
 * @param {string} imgPath
 * @param callback
 */
exports.byPath = function (imgPath, callback) {
    check.hasAccess(imgPath, err => {
        if (err) {
            callback(`Can not read file on path: ${imgPath}`, null);
        } else {
            callback(null, imgPath);
        }
    })
};


/**
 * Function takes a url, downloads the file and sends the full path to it.
 * @param {string} imgURI
 * @param callback
 */
exports.byURI = function (imgURI, callback) {
    const filename = imgURI.substring(imgURI.lastIndexOf('/') + 1);
    fetch(imgURI)
        .then(res => {
            const destination = fs.createWriteStream(filename);
            return res.body.pipe(destination)
        })
        .then((stream) => {
            stream.on('error', err => callback(err, null))
            stream.on('response', res => res.statusCode >= 400 ? callback('Something wrong with URL or server', null) : null)
            stream.on('finish', err => callback(err, `${process.cwd()}/${filename}`))
        })
};
