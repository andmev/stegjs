import { join } from 'path';
import { unlink } from 'fs/promises';

import * as GetFile from '../js/getFile';

describe('Get File Functionality', () => {
  test('it should allows user get file', async () => {
    const received = await GetFile.byPath('package.json');
    expect(received).toEqual(join(__dirname, '..', 'package.json'));
  });
  test('it should throw an error when pass wrong path', async () => {
    const received = (await GetFile.byPath(
      join(__dirname, '..', 'package.json'),
    )) as Error;
    expect(received.message).toContain('ENOENT: no such file or directory');
  });
  test('it should allow user get file by URL', async () => {
    const received = await GetFile.byURI(
      'https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png',
    );
    expect(received).toEqual(
      join(__dirname, '..', 'PNG_transparency_demonstration_1.png'),
    );
    await unlink(received);
  });
});
