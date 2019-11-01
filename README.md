# Rust Programming Language
My works related to rust programming language. Official website : https://www.rust-lang.org/

Rust is an open-source systems programming language created by Mozilla and a community of volunteers, designed to help developers create fast, secure applications which take full advantage of the powerful features of modern multi-core processors. It prevents segmentation faults and guarantees thread safety, all with an easy-to-learn syntax.

Further explanation about Rust programming language : https://developer.mozilla.org/en-US/docs/Mozilla/Rust

To create new hello-world cargo package; --bin means binary package.

```
cargo new hello-world --bin
```

To add Rust cargo folders into GitHub, it must be done, one by one.
```
git clone <name of the repository>
git status
git add <first folder> 
git add <second folder> 
git add <third folder>
git commit -m "message or remarks about the action done"
git push origin
git status
```

If there is any changes in the GitHub (remote) repository, first we need to pull the changes into our local machine, before adding the new content into GitHub
```
git status
git pull origin master
git add <file>
git commit -m "message or remarks about the action done"
git push origin
git status
```

###### GitHub formatting
https://help.github.com/en/github/writing-on-github/basic-writing-and-formatting-syntax
```diff
- text in red
+ text in green
! text in orange
# text in gray
```
https://primer.style/css/utilities/colors
- ![#f03c15](https://placehold.it/15/f03c15/000000?text=+) `#f03c15`
- ![#c5f015](https://placehold.it/15/c5f015/000000?text=+) `#c5f015`
- ![#1589F0](https://placehold.it/15/1589F0/000000?text=+) `#1589F0`

Add image into GitHub
https://www.youtube.com/watch?v=hHbWF1Bvgf4

First upload the image into the GitHub repository.

Use this command to upload the image directly into GitHub README.md content
```
![](image.jpg)
```

Or use this command to utilize HTML adjustment; the width and height will be automatically adjusted to appropriate size based on the pixels defined.
```
<img src="image.jpg" width="100">
```



