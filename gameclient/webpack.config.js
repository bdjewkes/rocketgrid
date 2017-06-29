var webpack = require('webpack');
var path = require("path");
module.exports = {
    entry: './src/index.tsx',
    output: {
        path: path.join(__dirname, "dist"),
        filename: 'bundle.js',
    },
    resolve: {
        extensions: ['.Webpack.js', '.web.js', '.ts', '.js', '.tsx']
    },
    module: {
        loaders: [
            {
                test: /\.tsx?$/,
                exclude: ["node_modules"],
                loader: 'ts-loader'
            },
        ]
    },
    plugins: [


    ],
    devServer: {
      contentBase: path.join(__dirname, 'dist'),
    }
}
