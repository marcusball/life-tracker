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
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/
            }
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js']
    }
};
