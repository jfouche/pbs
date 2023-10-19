# pbs : Product Breakdown Store

Allow you to manage a product breakdown

# Objectives

This software helps to design a solution which integrates multiple items.

# Tauri

## Dev
need `cargo` and `npm`
``` shell
cargo install --git https://github.com/DioxusLabs/cli
npm install -D tailwindcss
```

## Run
``` shell
cargo run
```


# pbs-cli commands

## Create new item :  `create <name>`
Create a new item in the store where :
- `<name>` is a name or label of the item  
A part number will be auto-generated

## Import existing item :  `import <pn> <name>`
Import an item to the store where :
- `<pn>` is the part number of the item (its reference)
- `<name>` is a name or label of the item  

# Example

```
import L289651 chair
import 305.294.67 table
create room
add-child 000001  L289651    25
add-child 000001  305.294.67 25
create kitchen
create restaurant
add-child 000020 L289651     80
add-child 000020 305.294.67  80
add 000100 School
add-child 000100 000001 7
add-child 000100 000010 1
add-child 000100 000020 1
```

