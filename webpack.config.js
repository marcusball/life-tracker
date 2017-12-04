const path = require('path');
const glob = require('glob');

const srcroot = './srcjs/';

module.exports = {
    // Pattern changed from "**/*.ts" to "**/!(*.d).ts" to omit compilation of ".d.ts" files. 
    // If I didn't place them in the /srcjs/ directory  like a dumbass, this wouldn't be necessary. 
    entry: glob.sync(srcroot + "**/!(*.d).ts").reduce((entries, entry) =>
        Object.assign(entries, {
            [entry.replace(srcroot, '').replace('.ts', '')]: entry
        }), {}),
    output: {
        path: __dirname + '/static/js',
        filename: '[name].bundle.js',
        publicPath: '/'
    },
    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",
    module: {
        rules: [{
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/
            },
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            {
                enforce: "pre",
                test: /\.js$/,
                loader: "source-map-loader"
            },
            // Add a handlebars loader to compile any handlebars templates
            {
                test: /\.hbs$/,
                loader: "handlebars-loader"
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js']
    }
};