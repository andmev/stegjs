import { TextDecoder, TextEncoder } from 'util';

/**
 * The function converts string into an array of bits.
 * @param str
 * @returns {Array}
 */
export const stringToBits = (str: string): boolean[] => {
  // Make a new buffer, based on our incoming message.
  const buffer = new TextEncoder().encode(str);

  // Let's make an array of bits, based on each byte in the buffer.
  const bitarray = [];

  for (let i = 0; i < buffer.length; i++) {
    let bytesString = buffer[i]?.toString(2);

    bytesString = bytesString?.padStart(8, '0');

    for (let k = 0; k < 8; k++) {
      if (bytesString?.charAt(k) === '0') {
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
export const bitsToString = (bits: number[]): string => {
  const buf = new Uint8Array(bits.length / 8);
  let byteID = -1;

  let myBytes = [];
  for (let i = 0; i < bits.length; i++) {
    myBytes.push(bits[i]);

    if (myBytes.length === 8) {
      let bytesString = '';
      for (let j = 0; j < 8; j++) {
        let useByte = '0';
        if (myBytes[j]) {
          useByte = '1';
        }
        bytesString = bytesString + useByte;
      }

      // Now convert that to an int.
      const ASCIINum = parseInt(bytesString, 2);

      byteID++;
      buf[byteID] = ASCIINum;

      // Clear it when done.
      myBytes = [];
    }
  }
  return new TextDecoder().decode(buf);
};

/**
 * Split meta-information.
 * @param str
 * @returns {Array|*}
 */
export const metaToObj = (str: string): [string, string, string] =>
  str.split('|') as [string, string, string];
