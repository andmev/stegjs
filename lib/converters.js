'use strict';

/**
 * The function converts string into an array of bits.
 * @param str
 * @returns {Array}
 */
exports.stringToBits = function (str) {

    // Make a new buffer, based on our incoming message.
    var buf = new Buffer(str);

    // Let's make an array of bits, based on each
    var bitarray = [];

    for (var i = 0; i < buf.length; i++) {


        var binstr = buf[i].toString(2);

        binstr = padZero(binstr);

        for (var k = 0; k < 8; k++) {
            if (binstr.charAt(k) == "0") {
                bitarray.push(false);
            } else {
                bitarray.push(true);
            }
        }
    }
    return bitarray;
};


/**
 * The function converts an array of bits into a readable string.
 * @param bits
 * @returns {string|String}
 */
exports.bitsToString = function (bits) {

    var buf = Buffer(bits.length / 8);
    var byteidx = -1;

    var mybyte = [];
    for (var i = 0; i < bits.length; i++) {
        mybyte.push(bits[i]);

        if (mybyte.length == 8) {


            var binstr = "";
            for (var j = 0; j < 8; j++) {
                var usebin = "0";
                if (mybyte[j]) {
                    usebin = "1";
                }
                binstr = binstr + usebin;
            }

            // Now convert that to an int.
            var asciinum = parseInt(binstr, 2);

            byteidx++;
            buf[byteidx] = asciinum;

            // Clear it when done.
            mybyte = [];
        }
    }
    return buf.toString();
};


/**
 * Split meta-information.
 * @param str
 * @returns {Array|*}
 */
exports.metaToObj = function (str) {
    return str.split('|');
};


/** Better to write own zero-pad :) */
function padZero(str) {
    for (var j = str.length + 1; j <= 8; j++) {
        str = "0" + str;
    }
    return str;
}