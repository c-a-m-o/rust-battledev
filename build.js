const fs = require('fs');
const { execSync } = require('child_process');
execSync("wasm-pack build --target nodejs")
const wasm_code_b64 = fs.readFileSync("pkg/rust_battledev_bg.wasm").toString('base64');
const template = fs.readFileSync("template.js").toString();
const out = template.replace('PLACEHOLDER', wasm_code_b64);
fs.writeFileSync("output.js", out);