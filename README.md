# BitAssets

## Install

Check out the repo with `git clone`, and then

```
git submodule update --init
cargo build --release
```

```
# if built from source
./target/release/plain_bitassets_app \
  --regtest \
  --l1-rpc-url http://127.0.0.1:18034 \
  --sidechain-id 1 \
  --data-dir /mnt/d/bitassets-regtest
```