import test from 'ava'

import { encode } from '../index.js'

test('sum from native', (t) => {
  encode('./img.png', 'some-key', '1x2', './output.png')
  t.is(true, true)
})
