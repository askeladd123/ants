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

function toRelUrlFromOutdir(outPath) {
  const unix = outPath.replaceAll(path.sep, '/')
  const rel = path.posix.relative(OUTDIR, unix)
  return `./${rel}`
}

async function writeManifestAndHtml(result, { hashed }) {
  const outputs = result.metafile?.outputs ?? {}
  const manifest = {}

  for (const outPath of Object.keys(outputs)) {
    const entryPoint = outputs[outPath].entryPoint
    if (entryPoint) {
      manifest[entryPoint] = toRelUrlFromOutdir(outPath)
    }
  }

  await fs.mkdir(OUTDIR, { recursive: true })
  await fs.writeFile(
    path.join(OUTDIR, 'manifest.json'),
    JSON.stringify(manifest, null, 2)
  )

  const tpl = await fs.readFile('src/index.html', 'utf8')
  const mainUrl = hashed ? (manifest[ENTRY] ?? './main.js') : './main.js'
  const html = tpl.replace('{{MAIN}}', mainUrl)

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
      write: true,
      logLevel: 'silent',
    })

    // force an initial build so we have a metafile
    const first = await ctx.rebuild()
    await writeManifestAndHtml(first, { hashed: false })

    await ctx.watch()
    await ctx.serve({ servedir: OUTDIR })
    break
  }
}
