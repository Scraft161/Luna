# MARSWM
This file contains the user documentation for the `marswm` window manager.

The [YAML](https://yaml.org/) format is used for configuration with the default file path being `~/.config/marswm/marswm.yaml`.
You can get the default configuration with `marswm print-default-config`.

## Multi-Monitor Setups and Workspaces
The window manager supports multi-monitor setups, although they are not as well tested as they probably should be for daily usage.
Every (non-overlapping) monitor gets its own set of workspaces, which is also exposed as such to other applications like status bars.
You can configure the number of the primary monitor and secondary monitors with the `primary_workspaces` and the `secondary_workspaces` option respectively.

It is suggested to use a relatively low number of workspaces for secondary monitors as they might clutter your bar otherwise.

## Layouts
`marswm` supports dynamic tiling and takes a lot of inspiration for it from [dwm](https://dwm.suckless.org).

Currently the following layouts are supported:
* `floating` - the clients are not automatically tiled in any way and can be freely positioned by the user
* `stack` - other windows are tiled vertically to the right of the main windows
* `bottom-stack` - other windows are tiled horizontally below the main windows
* `monocle` - all window are stacked on top of each other and fill the whole area
* `deck` - other windows are stacked to the right of the main windows on top of each other
* `dynamic` - this one is a little more complicated and is described in more detail down below

You can influence the layout of the windows with different parameters.
All of the following options belong in the `layout` section:
* `default` - specifies the default layout for new workspaces
* `gap_width` - size of the gap between windows and between the windowing area and the screen edge
* `main_ratio` - share of space that the main windows take on the screen
* `nmain` - how many windows the main area contains on a new workspace

Some of these values can be changed at runtime through respective keybindings.

### The `dynamic` Layout
As the name suggest the dynamic layout can be used to implement a variety of different layouts.
It is configured by these two parameters (also in the `layout` section of the configuration file):
* `stack_position` - specifies where the stack windows should be placed in relation to the main windows
* `stack_mode` - describes whether the stack windows should be in a `split` or `deck` configuration

## Theming
You can configure different parts of how `marswm` looks in the `theming` section of the configuration file.


These attributes influence the coloring of window borders:
* `primary_color`
* `secondary_color`
* `background_color`

*Note: Although they may look very weird in the output of `marswm print-default-config` colors can simply be written as hex values (like `0x1a2b3c`).*

Attributes specifying width are all in pixels:
* `frame_width`
* `inner_border_width`
* `outer_border_width`


## Keybindings
`marswm` comes with a set of default key bindings.
Call `marswm print-default-keybindings` to get an overview of them.

In contrast to the other sections of this manual the keybindings are not configured in the default configuration file.
Instead they are read from a separate YAML file (usually in `~/.config/marswm/keybindings.yaml`).
The bindings in that file will overwrite the default bindings.
If you wish to just extend the default key bindings by some custom ones you can use the file `~/.config/marswm/keybindings.yaml` which will then get merged with the default key bindings.

A key binding entry consists of a list of `modifers`, the `key` you want to bind as well as an `action` to execute as soon as a key is pressed.
Here is an example:
```YAML
- modifiers:
  - Mod4
  - Shift
  key: '1'
  action: !move-workspace 0
```

The actions are sadly not documented yet, but you can take a look at [the source code](src/bindings.rs).