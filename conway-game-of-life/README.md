## Refreence
https://rustwasm.github.io/docs/book/game-of-life/implementing.html

https://www.bilibili.com/video/BV1eg411g7c8?p=5&spm_id_from=pageDriver


Setup
```
  cargo install wasm-pack

  cargo install cargo-generate

  cargo generate --git https://github.com/rustwasm/wasm-pack-template

  cd <project-name>

  wasm-pack build  

  npm init wasm-app www  
```

modify package.json under www
```
 "dependencies": {
    "wasm-game-life" :"file:../pkg"
  },
```

modify index.js
```
import * as wasm from "wasm-game-life";
```

Run
```
  cd www

  npm i
  
  npm start 
```