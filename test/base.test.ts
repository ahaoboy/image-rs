import { decode } from '../dist'
import fs from 'node:fs'
import { test, expect, } from 'vitest'

test('decode jpeg', () => {
  const buf = fs.readFileSync('./assets/win.jpg')
  const { width, height, channel, pixels } = decode(buf)
  expect(width * height * channel).toEqual(pixels.length)
})
test('decode webp', () => {
  const buf = fs.readFileSync('./assets/win.webp')
  const { width, height, channel, pixels } = decode(buf)
  expect(width * height * channel).toEqual(pixels.length)
})
