import * as Converters from './converters';

describe('Converters Functionality', () => {
  test('', () => {
    // 0 to bit should be 00110000, so we should convert this
    // numbers into the boolean array
    const expectedResponse = '00110000'.split('').map((i) => !!Number(i));

    const response = Converters.stringToBits('0');

    expect(response).toStrictEqual(expectedResponse);
  });
  test('', () => {
    const testingBits = '00110000'.split('').map((i) => Number(i));
    const response = Converters.bitsToString(testingBits);
    expect(response).toBe('0');
  });
  test('', () => {
    const response = Converters.metaToObj('99|45|Hello World!');
    expect(response).toStrictEqual(['99', '45', 'Hello World!']);
  });
});
