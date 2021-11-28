const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'maple-uuid' is the name of native addon
 * the second arguments was decided by `napi.name` field in `package.json`
 * the third arguments was decided by `name` field in `package.json`
 * `loadBinding` helper will load `maple-uuid.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@napi-rs/maple-uuid-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'maple-uuid', '@napi-rs/maple-uuid')
