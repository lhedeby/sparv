<div align="center">
  <picture>
    <img width=100px height=100px alt="the logo of sparv" src="https://github.com/user-attachments/assets/d246756f-c2fe-432e-846f-2a9fe37f1c06" />
  </picture>
  <h1>Sparv</h1>
</div>

This is the main repo for the programming language Sparv. It is a dynamically typed interpreted language. It aims to be an easy-to-use, all-around scripting language to help
you with your needs.

## Installation

### Prerequisites
- [git](https://git-scm.com)
- [.NET 8](https://github.com/dotnet/core)

There are install scripts for powershell and linux. So to install just clone and run the corresponding script.

### Windows
```sh
    # Requires an elevated powershell prompt
    git clone https://github.com/lhedeby/sparv
    .\sparv\install.ps1
```
### Linux
```sh
    git clone https://github.com/lhedeby/sparv
    sudo chmod +x ./sparv/install.sh
    sudo ./sparv/install.sh
```
The ```sparv``` command should now be available from your terminal. If not - try opening a new terminal.  

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
