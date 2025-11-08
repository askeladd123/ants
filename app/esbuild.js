// esbuild.js
import * as esbuild from 'esbuild'
import { wasmLoader } from 'esbuild-plugin-wasm'
import fs from 'node:fs/promises'
import path from 'node:path'

const isBuild = process.argv[2] === 'build'
const isDev = process.argv[2] === 'dev'
if (!isBuild && !isDev) throw new Error('Usage: node esbuild.js <build|dev>')

const OUTDIR = 'www'
const ENTRY = 'src/main.js'

const base = {
  entryPoints: [ENTRY],
  outdir: OUTDIR,
  bundle: true,
  format: 'esm',
  plugins: [wasmLoader({ mode: 'deferred' })],
}

const hashedNames = {
  entryNames: '[name]-[hash]',
  chunkNames: 'chunks/[name]-[hash]',
  assetNames: 'assets/[name]-[hash]',
}

async function writeManifestAndHtml(result, { hashed }) {
  const metafile = result.metafile
  const outputs = metafile.outputs
  const manifest = {}

  for (const outPath of Object.keys(outputs)) {
    const entryPoint = outputs[outPath].entryPoint
    if (entryPoint) {
      // Normalize to web path under OUTDIR
      const rel = '/' + path.posix.relative(OUTDIR, outPath.replaceAll(path.sep, '/'))
      manifest[entryPoint] = rel
    }
  }

  await fs.writeFile(path.join(OUTDIR, 'manifest.json'), JSON.stringify(manifest, null, 2))

  const mainUrl = manifest[ENTRY] || '/main.js'
  const tpl = await fs.readFile('src/index.html', 'utf8')
  const html = tpl.replace('{{MAIN}}', hashed ? mainUrl : '/main.js')

  await fs.mkdir(OUTDIR, { recursive: true })
  await fs.writeFile(path.join(OUTDIR, 'index.html'), html)
}

switch (process.argv[2]) {
  case 'build': {
    const result = await esbuild.build({
      ...base,
      ...hashedNames,
      metafile: true,
      minify: true,
      sourcemap: false,
      write: true,
      logLevel: 'info',
    })
    await writeManifestAndHtml(result, { hashed: true })
    break
  }

  case 'dev': {
    const ctx = await esbuild.context({
      ...base,
      metafile: true,
      minify: false,
      sourcemap: true,
      define: { ESBUILD_LIVE_RELOAD: 'true' },
      // keep stable names in dev for simplicity
    })

    // Initial HTML for dev uses non-hashed /main.js
    await writeManifestAndHtml(
      { metafile: (await ctx.rebuild?.() ?? { metafile: { outputs: {} } }).metafile ?? { outputs: {} } },
      { hashed: false }
    )

    await ctx.watch()
    await ctx.serve({ servedir: OUTDIR })
    break
  }
}
