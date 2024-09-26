import * as Checker from '../js/checker';

describe('Checker Validation Functionality', () => {
  test('Extension should be .png', () => {
    const response = Checker.isPNG('cats.png');
    expect(response).toEqual(true);
  });
  test('Can not use *.webp *.gif *.jpeg', () => {
    const webpResponse = Checker.isPNG('cats.webp');
    const gifResponse = Checker.isPNG('cats.gif');
    const jpegResponse = Checker.isPNG('cats.jpeg');
    expect(webpResponse).toEqual(false);
    expect(gifResponse).toEqual(false);
    expect(jpegResponse).toEqual(false);
  });
  test('Should be the link', () => {
    const response = Checker.isURI('https://fbi.gov');
    expect(response).toEqual(true);
  });
  test('Can not use something else', () => {
    const nonHTTPSResponse = Checker.isURI('fbi.gov');
    const wrongResponse = Checker.isURI(',$.fbi.gov');
    expect(nonHTTPSResponse).toEqual(false);
    expect(wrongResponse).toEqual(false);
  });
  test('Should have an access to current folder', async () => {
    const response = await Checker.hasAccess('.');
    expect(response).toEqual(process.cwd());
  });
  test('Should be restricted for wrong file', () => {
    Checker.hasAccess('../../../file.txt').catch((e) => {
      expect(e.const).toEqual('ENOENT');
    });
  });
  test('Should allow use step with any numbers', () => {
    const responseSmall = Checker.isRightStep('99x55');
    const responseMiddle = Checker.isRightStep('829x9102');
    const responseLarge = Checker.isRightStep(`${2e15}x${7e20}`);
    expect(responseSmall).toEqual(['99', '55']);
    expect(responseMiddle).toEqual(['829', '9102']);
    expect(responseLarge).toEqual([
      '2000000000000000',
      '700000000000000000000',
    ]);
  });
  test('Should throw an error if wrong pattern', () => {
    try {
      Checker.isRightStep('99.55');
    } catch (e) {
      expect((e as Error).name).toEqual('Wrong step input. Check help!');
    }
  });
  test('Should throw an error if wrong pattern numbers', () => {
    try {
      Checker.isRightStep('AAxBB');
    } catch (e) {
      expect((e as Error).name).toEqual('Wrong step input. Check help!');
    }
  });
});
