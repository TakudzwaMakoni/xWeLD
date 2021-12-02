const Webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const Path = require("path");

module.exports = (env, args) => {
  const isProd = (args.mode === "production");

  return {
    entry: './index.js',
    output: {
      path: Path.resolve(__dirname, 'dist'),
      filename: isProd ? '[name].[contenthash].js' : '[name].[hash].js',
    },
    plugins: [
      new WasmPackPlugin({
        crateDirectory: Path.resolve(__dirname,'.')
      }),
      new HtmlWebpackPlugin({
        template: 'index.html'
      }),
      new Webpack.ProvidePlugin({
        TextDecoder: ['text-encoding', 'TextDecoder'],
        TextEncoder: ['text-encoding', 'TextEncoder'],
      })
    ],
    mode: isProd ? "production":"development",
    experiments: {
    asyncWebAssembly: true,
    },
  }
}
