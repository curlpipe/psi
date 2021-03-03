<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/curlpipe/psi">
    <h1 align="center">Ψ</h1>
  </a>

  <h2 align="center">PSI</h2>

  <p align="center">
     A minimal, sensible, scripting language for configuring, extending and controlling your application 
    <br />
    <a href="https://github.com/curlpipe/psi"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/curlpipe/psi">View Examples</a>
    ·
    <a href="https://github.com/curlpipe/psi/issues">Report Bug</a>
    ·
    <a href="https://github.com/curlpipe/psi/issues">Request Feature</a>
  </p>
</p>



<!-- TABLE OF CONTENTS -->
<details open="open">
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li><a href="#installation">Installation</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

PSI is a scripting language with the following aims:

- Be fast enough to be embedded with little slowdown
- Simple enough to use, but powerful enough to be practical
- Potentially be used as a general purpose scripting language
- Can be learnt in under 1 hour with prior experience.

### Built With

* [Rust](https://rust-lang.org) (Fast language to implement PSI in)
* [Lliw](https://github.com/curlpipe/lliw) (Provides colours and styles on the terminal)
* [Thiserror](https://github.com/dtolnay/thiserror) (Allows for easy custom error creation)
* [Unicode-rs](https://unicode-rs.github.io/) (For use to ensure unicode is supported)
* [Crafting Interpreters](https://craftinginterpreters.com) (Where I learnt to build it)

## Installation
<!--
### Binary
1. Install `curl` in your distro
2. Run the command
   
   ```sh
   curl -o /usr/bin/psi https://github.com/curlpipe/psi/release/...
   ```
3. You now have the `psi` executable ready to go
-->

### Source (Directly from Cargo)
1. Ensure you have an up-to-date Rust compiler toolchain (https://rust-lang.org)
2. Build from source (use a nightly compiler for potential performance improvements)
   
   ```sh
   cargo install psi-lang
   ```

### Source (Manually)
1. Ensure you have an up-to-date Rust compiler toolchain (https://rust-lang.org)
2. Clone the repo and enter the directory
   
   ```sh
   git clone https://github.com/curlpipe/psi.git
   cd psi/example
   ```
3. Build from source (use a nightly compiler for potential performance improvements)
   
   ```sh
   cargo build --release
   ```
4. Add the executable

   ```sh
   mv target/release/psibyte /usr/bin/psi
   ```

<!-- USAGE EXAMPLES -->
## Usage

You can find a cheatsheet for the language over [here](https://docs.rs/psi-lang)

You can also use the `--learn` or `-l` flag for an interactive learning environment to get yourself up to scratch on the langauge very quickly.

Here is the usage for the command line application itself:

- `psi -h` - Show help message
- `psi -r` - Access a REPL for trying out the language (a great environment to get to know it)
- `psi example.psi` - Run code from a file
- `psi example.psi -v` - Run code from a file (and show the internal workings of PSI)
- `psi -rv` or `psi -r -v` - Combine the repl and verbose argument to interactively show the internal workings
- `psi -l` - Access an interactive learning environment to learn the language quickly

<!-- LICENSE -->
## License

Distributed under the GPLv2 License. See `LICENSE` for more information.

<!-- CONTACT -->
## Contact

Project Link: [https://github.com/curlpipe/psi](https://github.com/curlpipe/psi)