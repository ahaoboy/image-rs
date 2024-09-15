import { bench, describe } from 'vitest'
import { encode_jpeg, decode_jpeg } from '../dist'
import fs from 'node:fs'
import { Transformer } from '@napi-rs/image'

const buf = fs.readFileSync('./assets/win.jpg')
const { pixels, width, height, channel } = decode_jpeg(buf)

describe("jpeg decode", () => {
  bench("wasm jpeg decode", () => {
    decode_jpeg(buf)
  })
  bench("napi jpeg decode", () => {
    new Transformer(buf).rawPixelsSync()
  })
})
