import * as esbuild from 'esbuild'
import { wasmLoader } from 'esbuild-plugin-wasm' // TODO: this is a thin wrapper, should not be necesarry. I did manage to run with `wasm-bindgen --target web`, but then files are not bundled

const config = {
  entryPoints: ['src/main.js'],
  outdir: 'www',
  bundle: true,
  format: 'esm',
  plugins: [wasmLoader({ mode: 'deferred' })],
}

switch (process.argv[2]) {
  case 'build': await esbuild.build({ ...config, minify: true, sourcemap: false }); break;
  case 'dev':
    let ctx = await esbuild.context({
      ...config,
      minify: false,
      sourcemap: true,
      define: { ESBUILD_LIVE_RELOAD: 'true' }
    })
    await ctx.watch()
    await ctx.serve({ servedir: 'www', });
    break;
  default: throw new Error('Usage: node esbuild.js <build|dev>');
}
