# Rust Programming Language
My works related to Rust programming language. 

<img src="rust.svg" width="100"> <img src="mozilla.jpg" width="100">

Rust is an open-source systems programming language created by Mozilla and a community of volunteers, designed to help developers create fast, secure applications which take full advantage of the powerful features of modern multi-core processors. It prevents segmentation faults and guarantees thread safety, all with an easy-to-learn syntax.

Rust was built from scratch and incorporates elements from tried-and-true systems programming languages and modern programming language design. It fuses the expressive and intuitive syntax of high-level languages with the control and performance of a low-level language. It also prevents segmentation faults and guarantees thread safety. This empowers developers to write code that is ambitious, fast and correct.

Rust makes systems programming accessible by combining power with ergonomics. Using it, programmers can make software that is less prone to bugs and security exploits. Under the hood, it includes powerful features such as zero-cost abstractions, safe memory management, fearless concurrency and more.

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

---

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

GitHub formatting
---

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

Rust Programming Language Books
---
<img src="rust books.jpg" width="500">
