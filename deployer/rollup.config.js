import rust from "@wasm-tool/rollup-plugin-rust";
import terser from '@rollup/plugin-terser';
import commonjs from '@rollup/plugin-commonjs';
import json from '@rollup/plugin-json';

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        deployer: "Cargo.toml",
    },
    output: {
        dir: "dist/js",
        format: "cjs",
        sourcemap: true,
        name: "deployer"
    },
    plugins: [
        json(),
        commonjs(),
        rust({
            serverPath: "dist/js/",
            nodejs: true,
        }),

        !is_watch && terser(),
    ],
    external: [
        '@cosmjs/cosmwasm-stargate',
        '@cosmjs/stargate',
        '@cosmjs/proto-signing',
        'crypto',
        'fs-extra',
        'path',
    ]
}