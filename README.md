<h1>Grey Script Compiler</h1>
<h3>Small file includer for Grey Script written in Rust 🦀</h3>
<h2>What is Grey Script?</h2>
Grey script is a fork of <a href="https://github.com/JoeStrout/miniscript"><b>Miniscript</b></a>. It is used in game named <a href="https://store.steampowered.com/app/605230"><b>Grey Hack</b></a>.
<h2> Features </h2>
<ul>
  <li><h3>Blazingly fast</li></h3>
  <li><h3>Pattern matching with <a href="https://en.wikipedia.org/wiki/Glob_(programming)">glob</a> </li></h3>
  <li><h3>Paste compiled version right in clipboard</li></h3>
</ul>

## Usage

```bash
greyscript_compiler --file <FILE_PATH> --output <OUTPUT_PATH>
```

### Paste to clipboard

```bash

greyscript_compiler --file <FILE_PATH> --clip
```

On <b>Linux</b> use xclip instead

```bash

greyscript_compiler --file <FILE_PATH> --output /dev/stdout | xclip -selection clipboard
```

## File example
Place glob pattern between //include< and > like shown below:

<b>a.gs</b>
```miniscript
a = function()
  print("Hello from a.gs!")
end function
```

<b>b.gs</b>
```miniscript
//include<a.gs>
print("Hello from b.gs!")
a // Hello from a.gs!
```

Another example:

<b>somefolder/a.gs</b>
```miniscript
a = function()
  print("Hello from a.gs!")
end function
```

<b>somefolder/another_folder/b.gs</b>
```miniscript
b = function()
  print("Hello from b.gs!")
end function
```

<b>c.gs</b>
```miniscript
//include<somefolder/**/*.gs>
// Will include any files with .gs extension in 'somefolder' and all it subfolders recursievly
a // Hello from a.gs!
b // Hello from b.gs!
```

<b>You can test your glob patterns in <a href="https://globster.xyz">globster.xyz</a></b>
