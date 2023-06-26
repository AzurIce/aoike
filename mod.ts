import { createServer, mergeConfig as mergeViteConfig } from 'npm:vite'
import vue from 'npm:@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'


import viteConfig from './vue-project/vite.config.ts'

// const root = "./vue-project"
console.log("Hello")

let server = await createServer({
    configFile: false, // 禁用 Vite 配置文件自动解析
    root: "./vue-project/",
    plugins: [
        vue()
    ],
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./vue-project/src', import.meta.url))
      }
    }
})
// let server = await createServer(viteConfig)
await server.listen()