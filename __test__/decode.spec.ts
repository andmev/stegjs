import test from 'ava'

import { decode } from '../index.js'

const dirname = new URL('.', import.meta.url).pathname

test('decode', async (t) => {
  await decode(dirname + 'decode.png')
  t.pass()
})

test('decode fails if image not exist', async (t) => {
  await t.throwsAsync(decode('fake.png'))
})
