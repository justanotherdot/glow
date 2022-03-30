# glow

## Running

Building this is currently fickle.

1. build the core shared object under resolvers/resolvers-core
2. build the neon bindings with `yarn` under resolvers
3. install the server's required dependencies with `yarn` at the top level

you can then run the server with `node server.js` and then try some
requests against it with `bin/playground`. With these changes, you can
rebuild the backing shared object in (1) and rerun queries without
taking the server down and get the updated answer, as long as there are
no changes to the interface.
