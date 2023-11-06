# Grey Script Compiler
### Small file includer for Grey Script written in Rust 🦀
## What is a Grey Script?
Grey script is a fork of [**Miniscript**](https://github.com/JoeStrout/miniscript). It's used in a game named [**Grey Hack**](https://store.steampowered.com/app/605230").

## Why do I even need this?
The game is very inconvenient to use the in-game code editor, but you can use external editors like [vs code](https://code.visualstudio.com/). They offer syntax checking and many other features. But when the complexity of the project grows, you need to split it into files. The game already has an **import_code** function that imports the contents of a file, but when using external editors, you have to copy and paste dozens of files inside the game. This program tries to solve this problem. With it you can compile all your code in a second and immediately insert it into the game.

## Features
- ### Blazingly fast
- ### Pattern matching with [glob](https://en.wikipedia.org/wiki/Glob_(programming))
- ### Paste the compiled version directly into your clipboard
- ### No need to split files
- ### Code compression (WIP)
</ul>

## Usage

```bash
greyscript_compiler --file <FILE_PATH> --output <OUTPUT_PATH>
```

### Paste into the clipboard

```bash

greyscript_compiler --file <FILE_PATH> --clip
```

On **Linux** use xclip instead

```bash

greyscript_compiler --file <FILE_PATH> --output /dev/stdout | xclip -selection clipboard
```

## File examples
Place the glob pattern between *//include<* and *>* like shown below:

**a.gs**
```miniscript
a = function()
  print("Hello from a.gs!")
end function
```

**b.gs**
```miniscript
//include<a.gs>
// Will include 'a.gs' file in directory where input file located

print("Hello from b.gs!")
a
```

Then, compile b.gs as shown below:
```bash
greyscript_compiler -f b.gs -o output.gs
```

**output.gs**
```miniscript
a = function()
  print("Hello from a.gs!")
end function


// Will include 'a.gs' file in directory where input file located

print("Hello from b.gs!")
a // Hello from a.gs!

```

Another example:

**somefolder/a.gs**
```miniscript
a = function()
  print("Hello from a.gs!")
end function
```

**somefolder/another_folder/b.gs**
```miniscript
b = function()
  print("Hello from b.gs!")
end function
```

**c.gs**
```miniscript
//include<somefolder/**/*.gs>
// Will include any files with .gs extension in 'somefolder' and all it subfolders recursively
a
b
```

Then, compile c.gs as shown below:
```bash
greyscript_compiler -f c.gs -o output.gs
```

**output.gs**
```miniscript
a = function()
  print("Hello from a.gs!")
end function

b = function()
  print("Hello from b.gs!")
end function


// Will include any files with .gs extension in 'somefolder' and all it subfolders recursively
a // Hello from a.gs!
b // Hello from b.gs!

```

You can have multiple includes anywhere in your input file

You can test your glob patterns [**here**](https://globster.xyz)


## Building from source
*Note for windows users:* you also need to download **[Builder Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16).** During installation, select **C++ tools**.

Requirements: [**rust**](https://www.rust-lang.org/tools/install)

1. <abbr title="git clone https://github.com/anarrak/greyscript_compiler">Clone</abbr> the repository first

2. In cloned repository directory run:
```bash
cargo build -r
```

3. You will find your executable file in target/release.

You can also download compiled executables in [**Releases**](https://github.com/anarrak/greyscript_compiler/releases) for linux and windows.

if you find a bug or something doesn't work, contact me on [discord](https://discord.com/users/711921484943327273).
