'use strict';

const conv = require('../lib/converters');

exports.conv = {
    setUp: function (callback) {
        this.arr = conv.stringToBits('some');
        callback()
    },
    stringToBits: function (test) {
        test.expect(1);
        test.ok(this.arr, 'should be array');
        test.done();
    },
    bitsToString: function (test) {
        test.ok(conv.bitsToString(this.arr), 'should be string');
        test.done();
    },
    metaToObj: function (test) {
        test.ok(conv.metaToObj('1|2|3'), 'should be array');
        test.done();
    }
};