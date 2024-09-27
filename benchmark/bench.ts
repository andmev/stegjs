import { Bench } from 'tinybench'

import { sum } from '../index.js'

function jsSum(a: number, b: number) {
  return a + b
}

const b = new Bench()

b.add('Native a + 100', () => {
  sum(10, 10)
})

b.add('JavaScript a + 100', () => {
  jsSum(10, 10)
})

await b.run()

console.table(b.table())
