'use strict';

const get = require('../lib/getFile');


exports.getFile = {
    errorByWrongURI: function (test) {
        get.byURI('https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png', (err) => {
            test.equal(err, 'Something wrong with URL or server');
        });
        test.done();
    },
    errorByWrongPath: function (test) {
        get.byPath('img1.png', (err) => {
            test.equal(err, 'Can not read file on path: img1.png');
        });
        test.done();
    }
};
