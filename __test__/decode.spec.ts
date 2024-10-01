import test from 'ava'
import path from 'path'

import { decode } from '../index.js'

test('decode', async (t) => {
  const result = await decode(path.join(process.cwd(), '__test__', 'decode.png'))
  t.pass()
  t.is(result.message, 'some-key')
  t.is(result.pattern, '1x1')
})

test('decode fails if image not exist', async (t) => {
  await t.throwsAsync(decode('fake.png'))
})
