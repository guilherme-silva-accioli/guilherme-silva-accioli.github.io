## Continuous Integration for WASM

Hosting a static WebAssembly application on GitHub Pages requires nothing more than building your artifacts into an output folder and transferring them to GitHub's CDN.

By using **GitHub Actions**, this entire process happens automatically on every push.

---

### The Workflow Blueprint

```yaml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - uses: jetli/trunk-action@v0.5.1
      - run: trunk build --release --public-url "/"
      - uses: actions/upload-pages-artifact@v3
        with:
          path: ./dist
      - uses: actions/deploy-pages@v4
```

### Essential Checklist

* Ensure the `wasm32-unknown-unknown` toolchain target is installed.
* Use `jetli/trunk-action` to download the pre-compiled Trunk binary instantly.
* Always copy `index.html` to `404.html` so client-side routing routes smoothly on GitHub Pages.
