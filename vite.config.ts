import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'
import AutoImport from 'unplugin-auto-import/vite'
import { loadEnv } from 'vite'
import babel from '@rollup/plugin-babel'

const mobile =
  process.env.TAURI_PLATFORM === 'android' ||
  process.env.TAURI_PLATFORM === 'ios'

import path from 'path'

// 获取当前项目路径
const root = process.cwd()

// https://vitejs.dev/config/
export default defineConfig(async (env) => {
  const viteEnv = loadEnv(env.mode, __dirname)
  return {
    plugins: [
      vue(),
      AutoImport({
        imports: ['vue'],
        dts: 'src/auto-import.d.ts',
      }),
      babel({
        exclude: /node_modules/, // 排除 node_modules 文件夹
        include: [/src\/.*\.js$/, /src\/.*\.ts$/], // 匹配 src 目录下所有 js 和 ts 文件
        babelHelpers: 'runtime',
        presets: [
          [
            '@babel/preset-env',
            {
              useBuiltIns: 'usage',
              corejs: 3,
            },
          ],
          '@babel/preset-typescript',
        ],
        plugins: [
          [
            '@babel/plugin-transform-runtime',
            {
              useESModules: true,
            },
          ],
        ],
      }),
    ],
    resolve: {
      //设置路径别名
      alias: {
        '@': resolve(__dirname, './src'),
      },
    },
    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    // prevent vite from obscuring rust errors
    clearScreen: false,
    // tauri expects a fixed port, fail if that port is not available
    server: {
      port: 1420,
      strictPort: true,
    },
    // to make use of `TAURI_DEBUG` and other env variables
    // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
    envPrefix: ['VITE_', 'TAURI_'],
    build: {
      // Tauri supports es2021
      target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
      // don't minify for debug builds
      minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
      // produce sourcemaps for debug builds
      sourcemap: !!process.env.TAURI_DEBUG,
    },
  }
})

