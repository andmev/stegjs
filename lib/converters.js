'use strict';

/**
 * The function converts string into an array of bits.
 * @param str
 * @returns {Array}
 */
exports.stringToBits = function (str) {

    // Make a new buffer, based on our incoming message.
    let buf = new Buffer.from(str);

    // Let's make an array of bits, based on each
    let bitarray = [];

    for (let i = 0; i < buf.length; i++) {


        let binstr = buf[i].toString(2);

        binstr = padZero(binstr);

        for (let k = 0; k < 8; k++) {
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

    let buf = Buffer.alloc(bits.length / 8);
    let byteidx = -1;

    let mybyte = [];
    for (let i = 0; i < bits.length; i++) {
        mybyte.push(bits[i]);

        if (mybyte.length == 8) {


            let binstr = "";
            for (let j = 0; j < 8; j++) {
                let usebin = "0";
                if (mybyte[j]) {
                    usebin = "1";
                }
                binstr = binstr + usebin;
            }

            // Now convert that to an int.
            let asciinum = parseInt(binstr, 2);

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
    for (let j = str.length + 1; j <= 8; j++) {
        str = "0" + str;
    }
    return str;
}
