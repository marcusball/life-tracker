const path = require('path');
const glob = require('glob');

const srcroot = './srcjs/';

module.exports = {
    entry: glob.sync(srcroot + "**/*.ts").reduce((entries, entry) =>
        Object.assign(entries, {
            [entry.replace(srcroot, '').replace('.ts', '')]: entry
        }),
        {}),
    output: {
        path: __dirname + '/static/js',
        filename: '[name].bundle.js',
        publicPath: '/'
    },
    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/
            },
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            { enforce: "pre", test: /\.js$/, loader: "source-map-loader" }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js']
    }
};
