import { bench, describe } from 'vitest'
import { encode_webp, decode_webp } from '../dist'
import fs from 'node:fs'
import { Transformer } from '@napi-rs/image'

const buf = fs.readFileSync('./assets/win.webp')

describe("webp decode", () => {
  bench("wasm webp decode", () => {
    decode_webp(buf)
  })
  bench("napi webp decode", () => {
    new Transformer(buf).rawPixelsSync()
  })
})
