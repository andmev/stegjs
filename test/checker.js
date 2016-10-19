'use strict';

const check = require('../lib/checker');

exports.checker = {
    isPNG: function (test) {
        test.ok(check.isPNG('img.png'), 'should be ok');
        test.done()
    },
    isURI: function (test) {
        test.ok(check.isURI('https://google.com/img.png'), 'should be ok');
        test.done()
    },
    isRightStep: function (test) {
        test.ok(check.isRightStep('1x1'), 'ok');
        test.done()
    }
};