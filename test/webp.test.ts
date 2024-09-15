import fs from 'node:fs'
import { encode_webp, decode,decode_webp } from '../dist'
import { test, expect, } from 'vitest'

test('webp', () => {
  const buf = fs.readFileSync('./assets/win.jpg')
  const { width, height, channel, pixels } = decode(buf)
  const webp = encode_webp(pixels, width, height, channel)
 const image =  decode_webp(webp)
 expect(image.width).toEqual(width)
 expect(image.height).toEqual(height)
 expect(image.channel).toEqual(channel)
})

