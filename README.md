# Rust Programming Language
My works related to Rust programming language.

<img src="rust.svg" width="100"> <img src="mozilla.jpg" width="100">

## Table of Contents
1. [Introduction.](#introduction)
2. [Official references websites.](#websites)
3. [Rust compiler : rustc.](#compiler)
4. [Rust package manager : Cargo.](#cargo)
5. [Regular expression : regex.](#regex)
6. [GitHub formatting.](#github)
7. [Rust Programming Language Books](#books)

<a name="introduction"></a>
## Introduction
Rust is an open-source systems programming language created by Mozilla and a community of volunteers, designed to help developers create fast, secure applications which take full advantage of the powerful features of modern multi-core processors. It prevents segmentation faults and guarantees thread safety, all with an easy-to-learn syntax.

Rust was built from scratch and incorporates elements from tried-and-true systems programming languages and modern programming language design. It fuses the expressive and intuitive syntax of high-level languages with the control and performance of a low-level language. It also prevents segmentation faults and guarantees thread safety. This empowers developers to write code that is ambitious, fast and correct.

Rust makes systems programming accessible by combining power with ergonomics. Using it, programmers can make software that is less prone to bugs and security exploits. Under the hood, it includes powerful features such as zero-cost abstractions, safe memory management, fearless concurrency and more.

<a name="websites"></a>
## Official references websites
Official website : https://www.rust-lang.org

Official development blog : https://blog.rust-lang.org

Official GitHub repository : https://github.com/rust-lang

Official conference (North America region) : https://rustconf.com

Official documentation : https://doc.rust-lang.org/1.8.0/book/index.html, https://devdocs.io/rust

Further explanation about Rust programming language : https://developer.mozilla.org/en-US/docs/Mozilla/Rust

Rust programming language is part of Mozilla Research : https://research.mozilla.org/rust

Mozilla wiki related to Rust programming language development : https://wiki.mozilla.org/Areweyet

Companies currently using Rust programming language : https://www.rust-lang.org/production/users

Companies hiring for Rust programming language project development : https://rustjobs.io

```
Mozilla Headquarters
331 E. Evelyn Ave.
Mountain View, CA 94041
United States of America
```
Mozilla Headquarters Map : https://goo.gl/maps/dy6SE

<a name="compiler"></a>
## Rust compiler : rustc
rustc is the compiler for the Rust programming language, provided by the project itself. Compilers take your source code and produce binary code, either as a library or executable. To check the version of your Rust compiler, simply type this below command.
```
$ rustc --version
```

<a name="cargo"></a>
## Rust package manager : Cargo
Cargo manages three things: building our code, downloading the dependencies our code needs, and building those dependencies. If you installed Rust via the official installers you will also have Cargo. To check the version of Cargo, simply type this below command.
```
$ cargo --version
```

To create new Rust cargo project, in this case hello-world cargo project package; --bin means binary package.
```
$ cargo new hello-world --bin
```

To open the newly created cargo project package folder, using Atom text-editor by GitHub, https://atom.io
```
$ atom .
```

To run the Rust cargo project, in this case hello-world cargo project package;
```
$ cd hello-world
$ cargo run
```

By default all **foldername\src\main.rs** will have this code inside them.
```
  fn main() {
    println!("Hello, world!");
}
```

To add Rust cargo folders into GitHub, it must be done, one by one, because there is .gitignore file in each folder.
```
$ git clone <name of the repository>
$ git status
$ git add <first folder> 
$ git add <second folder> 
$ git add <third folder>
$ git commit -m "message or remarks about the action done"
$ git push origin
$ git status
```

If there is any changes in the GitHub (remote) repository, first we need to pull the changes into our local machine, before adding the new content into GitHub
```
$ git status
$ git remote -v
$ git pull origin master
$ git add <file>
$ git commit -m "message or remarks about the action done"
$ git push origin
$ git status
```
For better view of the where your Git HEAD is located, try this commands below;
```
$ alias graph="git log --all --decorate --oneline --graph"
$ graph
```

If using Microsoft Windows PowerShell as the command line interpreter (CLI), then use this commands below;
In Atom, first you must install **_platformio-ide-terminal_** package to display the Powershell terminal in Atom by pressing **Ctrl + `** on your keyboard.
```
PS function graph { git log --all --decorate --oneline --graph }
PS graph
```

To only see the latest commit only, try this command, **-1** means only show the latest 1 commit
```
$ git log --oneline -1
```

To exit from the customized graph command, press **Q** button on your keyboard.

To see whether the file or folder already added into GitHub repository, refresh the GitHub web page, by hitting the **Ctrl + R** button on your keyboard, works on Google Chrome, Firefox Mozilla and Microsoft Edge browser.

To find or search back the file or folder that you already added into GitHub, press **Ctrl + F** on your keyboard, type the name of the file or folder that you already added into GitHub, and press **Enter**.

If you need to edit multiple same thing using Atom text editor, then you can try hit **Ctrl + Alt** buttons on your keyboard, mouse click on the selection, and start editing.

Rust has two distinct terms that relate to the module system: **crate** and **module**. A crate is synonymous with a ‘library’ or ‘package’ in other languages. Hence **_Cargo_** as the name of Rust’s package management tool: you ship your crates to others with Cargo. Crates can produce an executable or a library, depending on the project. Each crate has an implicit root module that contains the code for that crate.

You can opt in to a specific Rust Edition for your package with the edition key in **Cargo.toml**. If you don't specify the edition, it will default to 2015.

```
[package]
# ...
edition = '2018'
```

The edition key affects which edition your package is compiled with. Cargo will always generate packages via **_cargo new_** with the **edition** key set to the latest edition. Setting the edition key in **[package]** will affect all targets/crates in the package, including test suites, benchmarks, binaries, examples, etc.

Before editing the Cargo.toml file, first check for the crate dependencies using https://crates.io search inbox and type in the name of the crate to check the latest version of the crate before editing the Cargo.toml **[dependencies]** part. Refer to https://docs.rs documentation if using the previous version of dependencies, by typing the crate name you want to use in your Rust project.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules. By default, the items in a module have private visibility, but this can be overridden with the **pub** modifier. Only the public items of a module can be accessed from outside the module scope.

<a name="regex"></a>
## Regular expression : regex
A regular expression, regex or regexp, sometimes called a rational expression, is a sequence of characters that define a search pattern. Usually such patterns are used by string searching algorithms for "find" or "find and replace" operations on strings, or for input validation. It is a technique developed in theoretical computer science and formal language theory.

For Rust programming language regex, refer to the documentation : https://docs.rs/regex/1.3.1/regex/

A concept of regex by using Atom text editor, can refer to the explanation by **Corey Schafer** YouTube channel by the title **_Regular Expressions (Regex) Tutorial: How to Match Any Pattern of Text_**
https://www.youtube.com/watch?v=sa-TUpSx1JA&t=11s

<a name="github"></a>
## GitHub formatting
https://help.github.com/en/github/writing-on-github/basic-writing-and-formatting-syntax
```diff
- text in red
+ text in green
! text in orange
# text in gray
```

![#f03c15](https://placehold.it/15/f03c15/000000?text=+) `#f03c15 red` <br />
![#c5f015](https://placehold.it/15/c5f015/000000?text=+) `#c5f015`     <br />
![#1589F0](https://placehold.it/15/1589F0/000000?text=+) `#1589F0`     <br />
Colour selection, refer to : https://htmlcolorcodes.com

Add image into GitHub : https://www.youtube.com/watch?v=hHbWF1Bvgf4

First upload the image into the GitHub repository.

Use this command to upload the image directly into GitHub README.md content
```
![](image.jpg)
```

Or use this command to utilize HTML adjustment; the width and height will be automatically adjusted to appropriate size based on the pixels defined.
```
<img src="image.jpg" width="100">
```

List of GitHub markdown formatting
https://github.com/adam-p/markdown-here/wiki/Markdown-Cheatsheet

Table of Contents formatting for GitHub
```
1. [ Description. ](#description)
2. [ Usage. ](#usage)

<a name="description"></a>
## 1. Description
sometext

<a name="usage"></a>
## 2. Usage
sometext
```

<a name="books"></a>
## Rust programming language books
<img src="rust books.jpg" width="500">
