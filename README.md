<div align="center">
  <picture>
    <img width=100px height=100px alt="the logo of sparv" src="https://github.com/user-attachments/assets/d246756f-c2fe-432e-846f-2a9fe37f1c06" />
  </picture>
  <h1>Sparv</h1>
</div>

This is the main repo for the programming language Sparv. It is a dynamically typed interpreted language. It aims to be an easy-to-use, all-around scripting language to help
you with your needs.

## Installation
To get started you can build it yourself using [.NET](https://github.com/dotnet/core) 

```sh
    git clone https://github.com/lhedeby/sparv
    cd sparv/src
    dotnet build --configuration Release
```
After this you should add the build directory to your path. The following commands assumes your current directory is ```sparv/src```.
Feel free to move the executable and add the $PATH in any way you want. The following is just a suggestion:

### Windows
```pwsh
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "Machine")
    cd bin/Release/net8.0/
    $newPath = $currentPath + ";$pwd"
    [Environment]::SetEnvironmentVariable("Path", $newPath, "Machine")
    cd ../../..
```
### Linux
If you are using bash:
```sh
    cd bin/Release/net8.0/
    echo 'export PATH="$(pwd):$PATH"' >> ~/.bashrc
    source ~/.bashrc
    cd ../../..
```

## Getting started

For a guide and tips on getting started checkout [getting started](https://github.com/lhedeby/sparv/blob/main/docs/getting-started.md)

## Suggestions, errors and autocomplete - LSP

A simple implementation of [LSP](https://microsoft.github.io/language-server-protocol/) is bundled with this interpreter. Running ```sparv lsp``` starts the server
using stdio.

### Vscode

If you have installed sparv and added it to your path you just need to install the [exstension](https://marketplace.visualstudio.com/items?itemName=Sparv.sparv-lsp) and you should be ready to go!

### Neovim

If you are using [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) and have an on_attach function defined in your config you can add the following to your config:
```lua
    vim.filetype.add({
        extension = {
            sparv = "sparv",  -- Set the filetype for .sparv files
        },
    })
    vim.api.nvim_create_autocmd("FileType", {
        pattern = "sparv",
        callback = function()
            local client = vim.lsp.start_client {
                name = "sparvlsp",
                cmd = { "sparv", "lsp" },
                on_attach = on_attach
            }
            vim.lsp.buf_attach_client(0, client)
        end,
    })
```
