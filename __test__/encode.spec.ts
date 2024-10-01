import test from 'ava'
import fs from 'node:fs'
import path from 'path'

import { encode } from '../index.js'

const dirname = path.join(process.cwd(), '__test__')
const input = path.join(dirname, 'encode.png')
const output = path.join(dirname, 'output.png')

test.afterEach(() => {
  if (fs.existsSync(output)) {
    fs.unlinkSync(output)
  }
})

test('encode', async (t) => {
  const result = await encode(input, 'some-key', '1x1', output)
  t.timeout(10000, 'make sure the test is not timeout')
  t.is(fs.existsSync(output), true)
  t.is(result.pattern, '1x1')
  t.is(result.message, 'some-key')
  t.is(result.output, output)
})

test('encode fails if image not exist', async (t) => {
  await t.throwsAsync(encode('fake.png', 'wrong-key', '1x1', 'output.png'))
})
