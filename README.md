# Rust Programming Language
My works related to Rust programming language.

<img src="rust.svg" width="100"> <img src="mozilla.jpg" width="100">

## Table of Contents
1. [Introduction.](#introduction)
2. [Official references websites.](#websites)
3. [Rust installation.](#installation)
4. [Rust compiler : rustc.](#compiler)
5. [Rust package manager : Cargo.](#cargo)
6. [Rust library : crate.](#crate)
7. [Rust container : module](#module)
8. [Regular expression : regex.](#regex)
9. [GitHub Atom.](#atom)
10. [GitHub formatting.](#githubformatting)
11. [GitHub notes.](#githubnotes)
12. [Rust programming language books.](#books)
13. [GitHub repository calculation.](#calculation)

<a name="introduction"></a>
## 1. Introduction
Rust is an open-source systems programming language created by Mozilla and a community of volunteers, designed to help developers create fast, secure applications which take full advantage of the powerful features of modern multi-core processors. It prevents segmentation faults and guarantees thread safety, all with an easy-to-learn syntax.

Rust was built from scratch and incorporates elements from tried-and-true systems programming languages and modern programming language design. It fuses the expressive and intuitive syntax of high-level languages with the control and performance of a low-level language. It also prevents segmentation faults and guarantees thread safety. This empowers developers to write code that is ambitious, fast and correct.

Rust makes systems programming accessible by combining power with ergonomics. Using it, programmers can make software that is less prone to bugs and security exploits. Under the hood, it includes powerful features such as zero-cost abstractions, safe memory management, fearless concurrency and more.

<a name="websites"></a>
## 2. Official references websites
Official website : https://www.rust-lang.org

Online Rust playground : https://play.rust-lang.org

Official development blog : https://blog.rust-lang.org

Official GitHub repository : https://github.com/rust-lang

Official conference (North America region) : https://rustconf.com

Official documentation : https://doc.rust-lang.org/1.8.0/book/index.html, https://devdocs.io/rust

Further explanation about Rust programming language : https://developer.mozilla.org/en-US/docs/Mozilla/Rust

Rust programming language is part of Mozilla Research : https://research.mozilla.org/rust

Mozilla wiki related to Rust programming language development : https://wiki.mozilla.org/Areweyet

Companies currently using Rust programming language : https://www.rust-lang.org/production/users

Companies hiring for Rust programming language project development : https://rustjobs.io

Most active Rust developers in GitHub : http://git-awards.com/users?language=rust

Rust programming language was created by Graydon Hoare : https://github.com/graydon

```
Mozilla Headquarters
331 E. Evelyn Ave.
Mountain View, CA 94041
United States of America
```
Mozilla Headquarters Map : https://goo.gl/maps/dy6SE

**_Rust related articles_**
Zero-cost futures in Rust : http://aturon.github.io/blog/2016/08/11/futures/ <br />
Developer Survey Results 2019 : https://insights.stackoverflow.com/survey/2019 <br />

<a name="installation"></a>
## 3. Rust installation
To install Rust in Linux operating system, open up the terminal by pressing **[Ctrl]** + **[Alt]** + **[T]** and type this command.
```
$ curl https://sh.rustup.rs -sSf | sh
```

To install Rust in Microsoft Windows operating system, download `rustup-init.exe` file from https://www.rust-lang.org/tools/install

<a name="compiler"></a>
## 4. Rust compiler : rustc
rustc is the compiler for the Rust programming language, provided by the project itself. Compilers take your source code and produce binary code, either as a library or executable. To check the version of your Rust compiler, simply type this below command.
```
$ rustc --version
```
If an error message `error: linker ``cc`` not found` appears after compiling Rust project then do this command.
```
$ sudo apt-get install -y gcc-4.8 cpp-4.8 gcc-multilib
```

<a name="cargo"></a>
## 5. Rust package manager : Cargo
Cargo manages three things: building our code, downloading the dependencies our code needs, and building those dependencies. If you installed Rust via the official installers you will also have Cargo. To check the version of Cargo, simply type this below command.
```
$ cargo --version
```

To create new Rust cargo project, in this case hello-world cargo project package; --bin means binary package.
```
$ cargo new hello-world --bin
```

To open the newly created cargo project package folder, in this case **hello-world** folder, using Atom text-editor by GitHub, https://atom.io
```
$ atom hello-world
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

<a name="crate"></a>
## 6. Rust library : crate
A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

Before editing the Cargo.toml file, first check for the crate dependencies using https://crates.io search inbox and type in the name of the crate to check the latest version of the crate before editing the Cargo.toml **[dependencies]** part. Refer to https://docs.rs documentation if using the previous version of dependencies, by typing the crate name you want to use in your Rust project.

<a name="module"></a>
## 7. Rust container : module
A module is a container for zero or more items. A module item is a module, surrounded in braces, named, and prefixed with the keyword mod. A module item introduces a new, named module into the tree of modules making up a crate. Modules can nest arbitrarily.

A module is a collection of items: functions, structs, traits, impl blocks, and even other modules. By default, the items in a module have private visibility, but this can be overridden with the **pub** modifier. Only the public items of a module can be accessed from outside the module scope.

<a name="regex"></a>
## 8. Regular expression : regex
A regular expression, regex or regexp, sometimes called a rational expression, is a sequence of characters that define a search pattern. Usually such patterns are used by string searching algorithms for "find" or "find and replace" operations on strings, or for input validation. It is a technique developed in theoretical computer science and formal language theory.

For Rust programming language regex, refer to the documentation : https://docs.rs/regex/1.3.1/regex/

List of usual regex expressions : https://projects.lukehaas.me/regexhub/

A concept of regex by using Atom text editor, can refer to the explanation by **Corey Schafer** YouTube channel by the title **_Regular Expressions (Regex) Tutorial: How to Match Any Pattern of Text_** : 
https://www.youtube.com/watch?v=sa-TUpSx1JA&t=11s

<a name="atom"></a>
## 9. GitHub Atom
Atom is a free and open-source text and source code editor for macOS, Linux, and Microsoft Windows with support for plug-ins written in Node.js, and embedded Git Control, developed by GitHub. Atom is a desktop application built using web technologies. Most of the extending packages have free software licenses and are community-built and maintained. Atom is based on Electron (formerly known as Atom Shell), a framework that enables cross-platform desktop applications using Chromium and Node.js. It is written in CoffeeScript and Less.

Official website of GitHub atom : https://atom.io

If you want to close all the tabs in Atom, press [Ctrl] + [K], then [Ctrl] + [W] on your keyboard.

<a name="github"></a>
## 10. GitHub formatting
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

<a name="githubnotes"></a>
## 11. GitHub notes.
Clone the current GitHub remote repository contents into local machine.
```
$ git clone https://github.com/syakirharis25/rust.git
$ cd rust/
$ git remote -v
$ git status
```

<a name="books"></a>
## 12. Rust programming language books
<img src="rust books.jpg" width="500">

**The Rust Programming Language** by Steve Klabnik and Carol Nichols <br />
**Programming Rust: Fast, Safe Systems Development** by Jim Blandy and Jason Orendorff <br />

<a name="calculation"></a>
## 13. GitHub repository calculation.
```
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
D                              444            946              0           3810
JSON                           580              0              0            580
Rust                            43            136             94            487
TOML                            40             80             40            246
Markdown                         1             62              0            194
Python                           1              0              0              1
SVG                              1              0              0              1
-------------------------------------------------------------------------------
SUM:                          1110           1224            134           5319
-------------------------------------------------------------------------------
```
Refer to : https://github.com/syakirharis25/cloc
