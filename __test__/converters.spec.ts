import * as Converters from '../js/converters'

describe('Converters Functionality', () => {
  test('stringToBits should convert "0" to a boolean array', () => {
    // 0 to bit should be 00110000, so we should convert this
    // numbers into the boolean array
    const expectedResponse = '00110000'.split('').map((i) => !!Number(i))

    const response = Converters.stringToBits('0')

    expect(response).toStrictEqual(expectedResponse)
  })

  test('bitsToString should convert a boolean array to "0"', () => {
    const testingBits = '00110000'.split('').map((i) => Number(i))
    const response = Converters.bitsToString(testingBits)
    expect(response).toBe('0')
  })

  test('metaToObj should split the string into an array', () => {
    const response = Converters.metaToObj('99|45|Hello World!')
    expect(response).toStrictEqual(['99', '45', 'Hello World!'])
  })
})
