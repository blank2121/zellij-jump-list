# Zellij Jump List

A [Zellij](https://zellij.dev) plugin for navigating your motions from pane-to-pane.

Inspired by the jump list commonly in editors like vim, nvim, emacs.

![usage](add a link here to and image)

## Usage

- `Up` and `Down` or `j` and `k` to cycle through the jump pane list
- `Enter` to go back to the selected pane
- `Esc` to exit

## Why?

Briefly: to quickly go to previous panes.

- Can jump to old panes from different tabs.
- Easy to use.

## Installation

**Requires Zellij `0.38.0` or newer.**

*Note*: you will need to have `wasm32-wasi` added to rust as a target to build the plugin. This can be done with `rustup target add wasm32-wasi`.

```bash
git clone https://github.com/blank2121/zellij-jump-list.git
cd harpoon
./install.sh
```
> If `install.sh` does not run or does not have the permission to run, run `chmod +x ./install.sh`

All `./install.sh` does is compile it and move the .wasm to `~/.config/zellij/plugins/`

## Keybinding

Add the following to your [zellij config](https://zellij.dev/documentation/configuration.html)
somewhere inside the [keybinds](https://zellij.dev/documentation/keybindings.html) section:

```kdl
shared_except "locked" {
    bind "Ctrl j" {
        LaunchOrFocusPlugin "file:~/.config/zellij/plugins/zellij-jump-list.wasm" {
            floating true; move_to_focused_tab true;
        }
    }
}
```

> You likely already have a `shared_except "locked"` section in your configs. Feel free to add `bind` there.

## Contributing

If you find any issues or want to suggest ideas please [open an issue](https://github.com/blank2121/zellij-jump-list/issues/new).
