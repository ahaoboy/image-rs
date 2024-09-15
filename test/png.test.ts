import fs from 'node:fs'
import { encode_png, decode, decode_png } from '../dist'
import { test, expect, } from 'vitest'

test('png', () => {
  const buf = fs.readFileSync('./assets/win.jpg')
  const { width, height, channel, pixels } = decode(buf)
  const webp = encode_png(pixels, width, height, channel)
  const image = decode_png(webp)
  expect(image.width).toEqual(width)
  expect(image.height).toEqual(height)
  expect(image.channel).toEqual(channel)
})

