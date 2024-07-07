<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/RustFetch/RustFetch/main/images/rustfetch_full_dark_mode.png">
  <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/RustFetch/RustFetch/main/images/rustfetch_full.png">
  <img alt="RustFetch Header" src="https://raw.githubusercontent.com/RustFetch/RustFetch/main/images/rustfetch_full.png">
</picture>

<br>
<p></p>
<br>

<div align="center">
  <code>RustFetch</code> is a fast, crossplatform *Fetch aiming to provide full feature parity with <code>NeoFetch</code> and more, made in <code>Rust</code>.
  <br>
  <b>Currently in Development</b>
</div>

<hr>

<div>
  <img src="https://github.com/RustFetch/RustFetch/actions/workflows/rust-build.yml/badge.svg" alt="Actions">
  <img src="https://img.shields.io/badge/Alpha-0.1.0-orange" alt="Version - 0.1.0">
  <img src="https://img.shields.io/badge/License-MIT-yellow" alt="License - MIT">
</div>

<br>
<p></p>
<br>

<div>
  <h3>About RustFetch</h3>
  
  Our aim is to one day provide at least a full alternative to the popular <code>NeoFetch</code> while being crossplatform, fast and reliable.
  To achieve this we are remaking <code>NeoFetch</code> in <code>Rust</code>, which is a mayor upgrade compared to the shell scripting languages regular *Fetches use.
  While we currently only have a fraction of the features we value and take in outside contributions! So feel free to help!
</div>

<br>
<p></p>
<br>

<div>
  <h3>Index</h3>
  
  <ul>
    <li><a href="https://github.com/RustFetch/RustFetch/releases">Releases</a></li>
    <li><a href="#screenshots">Screenshots</a></li>
    <li><a href="#features">Features</a></li>
    <li><a href="#building">Building</a></li>
  </ul>
</div>

<br>
<p></p>
<br>

<div>
  <h3 id="screenshots">Screenshots</h3>
  
  <img src="https://github.com/RustFetch/RustFetch/assets/112782958/dd8ae54f-ff91-4dfd-9a4f-086f5c8e35de" align="right">

  <br clear="right">
  <p></p>
  <br clear="right">
  
  <img src="https://github.com/RustFetch/RustFetch/assets/112782958/45044daf-5119-4b3e-a23c-2fb8279ac6f5">
</div>

<br>
<p></p>
<br>

<div>
  <h3 id="features">Features</h3>
  
  We currently support:
  
  <ul>
    <li>ASCII Images</li>
    <li>Getting the current user and host</li>
    <li>OS and kernel information</li>
    <li>Uptime</li>
    <li>The current shell</li>
    <li>CPU (+ Multi-CPU)</li>
    <li>Used and total RAM</li>
    <li>Terminal color display</li>
  </ul>

  Currently planned is (not in order):

  <ul>
    <li>Package manager information</li>
    <li>Expanding shell information</li>
    <li>Fancier OS text</li>
    <li>GPU information (possibly NPU)</li>
    <li>Showing the current WM/DE</li>
    <li>Full image support</li>
    <li>Customization via CLI arguments and a config file</li>
  </ul>
</div>

<br>
<p></p>
<br>

<div>
  <h3 id="building">Building</h3>

  First make sure <a href="https://rustup.rs/"><code>rustup</code></a> is installed and updated, we are on the <code>stable</code> toolchain with rustc being on at least 1.79.0:
  ```bash
    $ rustup update
    $ rustup default stable
    $ rustup show
    active toolchain
    ----------------
    stable-(x) (default)
    rustc 1.79.0 (129f3b996 2024-06-10)
  ```

  Navigate to the projects directory.
  For a development build run:
  ```bash
    $ cargo run
  ```

  To build a production ready executable run:
  ```bash
    $ cargo build --release
  ```
  The executable should now be inside of <code>./target/release/rust_fetch(.exe)</code>
</div>
