const { cpSync, rmSync } = require('fs');
const { execSync } = require('child_process');

const cordova_root = 'src-cordova/';

const dst_dir = 'www/';

const assets_dir = 'assets/';

const css_src = 'index.css';
const css_dir = dst_dir + 'css/';

const wasm_dir = dst_dir + 'js/';
const wasm_build_cmd = 'wasm-pack build --release \
    --target=web \
    --no-typescript \
    --out-dir=';

module.exports = function (context) {
    // Copy assets folder
    console.log('Copying `assets` folder to ' + dst_dir + assets_dir);
    cpSync('../' + assets_dir, dst_dir + assets_dir, { recursive: true });

    // Copy CSS file
    console.log('Copying `' + css_src + '` to `' + css_dir + css_src);
    cpSync('../' + css_src, css_dir + css_src);

    // Build WASM files
    console.log('Building WASM module...');
    execSync(wasm_build_cmd + cordova_root + wasm_dir, {'cwd': '../'});
    console.log('Removing excess files...');
    rmSync(wasm_dir + 'package.json');
}
