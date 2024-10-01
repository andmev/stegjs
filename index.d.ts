/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface EncodeResult {
  output: string
  message: string
  pattern: string
}
export interface DecodeResult {
  message: string
  pattern: string
}
/** Exposes the `encode_rs` function to Node.js via N-API. */
export declare function encode(image: string, message: string, step: string, output: string): Promise<EncodeResult>
/** Exposes the `decode_rs` function to Node.js via N-API. */
export declare function decode(image: string): Promise<DecodeResult>
