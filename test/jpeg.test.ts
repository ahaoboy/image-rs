import fs from 'node:fs'
import { encode_jpeg, decode, decode_jpeg } from '../dist'
import { test, expect, } from 'vitest'

test('jpeg', () => {
  const buf = fs.readFileSync('./assets/win.jpg')
  const { width, height, channel, pixels } = decode(buf)
  const webp = encode_jpeg(pixels, width, height, channel)
  const image = decode_jpeg(webp)
  expect(image.width).toEqual(width)
  expect(image.height).toEqual(height)
  expect(image.channel).toEqual(channel)
})

