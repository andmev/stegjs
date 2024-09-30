import fs from 'node:fs'
import test from 'ava'

import { encode } from '../index.js'

const dirname = new URL('.', import.meta.url).pathname

test.afterEach(() => {
  const output = dirname + 'output.png'
  if (fs.existsSync(output)) {
    fs.unlinkSync(output)
  }
})

test('encode', async (t) => {
  await encode(dirname + 'encode.png', 'some-key', '1x1', dirname + 'output.png')
  t.timeout(10000, 'make sure the test is not timeout')
  t.is(fs.existsSync(dirname + 'output.png'), true)
})

test('encode fails if image not exist', async (t) => {
  await t.throwsAsync(encode('fake.png', 'wrong-key', '1x1', 'output.png'))
})
