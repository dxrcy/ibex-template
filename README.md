# Ibex Template

Simple template for an [Ibex](https://github.com/dxrcy/ibex) SSG site.

- [Live on GitHub Pages](https://dxrcy.github.io/ibex-template)

# Usage

```sh
# Clone the repo
git clone https://github.com/dxrcy/ibex-template my-ibex-app
cd my-ibex-app
git remote remove origin

# Install some dev dependencies
cargo install just cargo-watch basic-http-server

# Build and open a local server on localhost:4000
just serve
```

## GitHub Pages Setup

### Deploy from branch (Default method)

GitHub will automatically build to the `gh-pages` branch, with the `deploy.yaml` Action. The action will automatically run when the repository is initially cloned.

In repository settings, navigate to the `Pages` tab, and change 'Branch' to `gh-pages`. GitHub pages will automatically update, and the website should be live soon.

### *Beta* GitHub Action (Alternate method)

> This section is missing, but you can see [https://github.com/dxrcy/garfeo](this repository) for a working example.

# File structure

No Rust code is included in the website itself. It is just for compilation.

- Everything is compiled to `/build` (except `/target`)
- Source files are included in `/src`
    - `/src` is the Rust binary source folder
    - `/src/scss` will be compiled to css and written to `/build/css`
    - `/src/js` can be used to `include_str!` Javascript code into templates
- Static files (such as images or assets) are found in `/static`, and are copied directly into `/build/static`
    - `/static` may be symlinked in development mode (for compilation speed), but never in production

![Ibex logo](static/icon.png)

