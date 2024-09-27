import test from 'ava'

import { encode } from '../index.js'

test('sum from native', (t) => {
  t.is(encode('hello'), 'aGVsbG8=')
})
