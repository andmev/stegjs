import test from 'ava'
import path from 'path'

import { decode } from '../index.js'

test('decode', async (t) => {
  await decode(path.join(process.cwd(), '__test__', 'decode.png'))
  t.pass()
})

test('decode fails if image not exist', async (t) => {
  await t.throwsAsync(decode('fake.png'))
})
