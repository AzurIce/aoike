import { createServer, mergeConfig as mergeViteConfig } from 'npm:vite'
import vue from 'npm:@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import { options, blue, bold, cyan, dim, gray, green, underline, yellow } from 'npm:kolorist'


import viteConfig from './vue-project/vite.config.ts'

// const root = "./vue-project"
console.log("Hello")
// Deno.env.set("NODE_ENV", "development")
// console.log(Deno.env.get("NODE_ENV"))

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
printInfo()

function printInfo() {
    options.enabled = true;
    console.log()
    console.log(`  ${bold('Aoike')}`)
    console.log()
    console.log(`${dim('  Preview   ')} > ${cyan(`http://localhost:127.0.0.1/`)}`)
    console.log()
}