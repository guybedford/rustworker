## Self-hosted Rust Worker Example

This implements the Durable Objects getting started example (0https://developers.cloudflare.com/durable-objects/get-started/),
using wasm-bindgen's latest functionality to build a worker from Rust without requiring any custom JS wrapping.

To try it locally:

```
git clone --recurse-submodules https://github.com/guybedford/rustworker
cd rustworker
npx wrangler dev
```

## How it works

We use snippets, reexports and the new free-function `this` functionality in Wasm Bindgen to fully self-host a Wasm module
matching the shape of the following JS:

```js
import { DurableObject } from "cloudflare:workers";

export class MyDurableObject extends DurableObject {
  async sayHello() {
    let result = this.ctx.storage.sql
      .exec("SELECT 'Hello, World!' as greeting")
      .one();
    return result.greeting;
  }
}

export default {
  async fetch(request, env, ctx) {
    const stub = env.MY_DURABLE_OBJECT.getByName(new URL(request.url).pathname);

    const greeting = await stub.sayHello();

    return new Response(greeting);
  },
};
```

See [src/lib.rs](src/lib.rs) for how the above is represented in Rust in a 1-1 relationship.
