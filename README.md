<p align="center">
  <img
    width="400"
    src="https://raw.githubusercontent.com/piquette/quantlib/master/media/logo.png"
    alt="QuantLib – Rust"
  />
</p>

<p align="center">

 <!-- Crates version -->
  <a href="https://crates.io/crates/quantlib">
    <img src="https://img.shields.io/crates/v/quantlib.svg"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/quantlib">
    <img src="https://img.shields.io/crates/d/quantlib.svg"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/quantlib">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg"
      alt="docs.rs docs" />
  </a><br />
    <a href="https://travis-ci.org/piquette/quantlib">
      <img src="https://travis-ci.org/piquette/quantlib.svg?branch=master"
      alt="Build Status">
    </a>
  <img src="https://img.shields.io/crates/l/quantlib.svg" alt="license">
</p>

<p align="center">
  <a href="https://crates.io/crates/quantlib">Crate</a>
  ·
  <a href="#🚀-installation">Installation</a>
  ·
  <a href="https://crates.io/crates/quantlib">Configuration</a>
</p>

<h1></h1>

**The idiomatic Rust implementation of QuantLib, the standard for professional quantitative finance applications.**

**This project is still under active development and not guaranteed to have a stable API.**

**The alpha release version is aiming for:**

- **Familiar interfaces:** A library api that resembles the original.
- **High test coverage:** 90% or more.
- **Idiomatic rust:** architecture that takes full advantage of language features.
- **Easy:** intuitive to understand coupled with extensive documentation. 🚀

<h1></h1>


<a name="💬-about"></a>

## 💬 About 

The QuantLib project (http://quantlib.org) is aimed at providing a comprehensive software framework for quantitative finance. QuantLib is a free/open-source library for modeling, trading, and risk management in real-life.

Appreciated by quantitative analysts and developers, it is intended for academics and practitioners alike, eventually promoting a stronger interaction between them. QuantLib offers tools that are useful both for practical implementation and for advanced modeling, with features such as market conventions, yield curve models, solvers, PDEs, Monte Carlo (low-discrepancy included), exotic options, VAR, and so on.



<a name="🚀-installation"></a>

## 🚀 Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
quantlib = "0.1.0"
```

and this to your crate root:

```rust
extern crate quantlib;
```

