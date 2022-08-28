const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
    entry: "./bootstrap.ts",
    output: {
        path: path.resolve(__dirname, "../public"),
        filename: "bootstrap.js",
    },
    mode: "development",
    module: {
        rules: [
          {
            test: /\.tsx?$/,
            use: 'ts-loader',
            exclude: /node_modules/,
          },
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                {
                    from: path.resolve(__dirname, "./index.html"),
                    to: path.resolve(__dirname, "./public")
                }
            ]
        })
    ],
}