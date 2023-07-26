import { Command } from "https://deno.land/x/cliffy@v1.0.0-rc.2/command/mod.ts";

import { createServer, build, mergeConfig as mergeViteConfig, InlineConfig } from 'vite'
import vue from 'npm:@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'node:url'
import { options, blue, bold, cyan, dim, gray, green, underline, yellow } from 'npm:kolorist'

const viteConfig: InlineConfig = {
  configFile: false, // 禁用 Vite 配置文件自动解析
  root: "./test",
  plugins: [
    {
      name: "vite-plugin-aoike",
      configResolved(resolvedConfig) {
        // 存储最终解析的配置
        // console.log(resolvedConfig)
      },
      configureServer(server) {
        // server.middlewares.use((req, res, next) => {
        //   console.log(req)
        // })
      },
      transform(src, id) {
        console.log(`transform: ${id}`)
        if (id.endsWith('.md')) {
          return {
            code: `export default ${JSON.stringify(src)}`,
            map: null,
          };
        }
        // const fileRegex = /\.(md)$/
        // if (fileRegex.test(id)) {
        //   return {
        //     code: '1',
        //     map: null // 如果可行将提供 source map
        //   }
        // }
      },
    }
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./vue-project/src', import.meta.url))
    }
  }
}

await new Command()
  .name("aoike")
  .version("0.1.0")
  .description("A static blog framework")
  .command(
    "dev",
    "Serve a static blog"
  )
  .action(async () => {
    console.log("dev")

    const server = await createServer(viteConfig)
    await server.listen()
    printInfo()

    function printInfo() {
      options.enabled = true;
      console.log()
      console.log(`  ${bold('Aoike')}`)
      console.log()
      console.log(`${dim('  Preview   ')} > ${cyan(`http://localhost:5173/`)}`)
      console.log()
    }
  })
  .command(
    "build",
    "Build the site"
  )
  .action(async () => {
    console.log("build")

    await build(viteConfig)
  })
  .parse(Deno.args)

// import viteConfig from './vue-project/vite.config.ts'

// const root = "./vue-project"
// console.log("Hello")
// Deno.env.set("NODE_ENV", "development")
// console.log(Deno.env.get("NODE_ENV"))

// let server = await createServer({
//     configFile: false, // 禁用 Vite 配置文件自动解析
//     root: "./vue-project/",
//     plugins: [
//         vue()
//     ],
//     resolve: {
//       alias: {
//         '@': fileURLToPath(new URL('./vue-project/src', import.meta.url))
//       }
//     }
// })
