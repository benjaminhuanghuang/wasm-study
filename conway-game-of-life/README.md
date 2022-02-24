
https://rustwasm.github.io/docs/book/game-of-life/implementing.html


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