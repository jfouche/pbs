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

## Create new item :  `make <name>`
Create a new item in the store where :
- `<name>` is a name or label of the item  
A part number will be auto-generated

## Import existing item :  `buy <pn> <name>`
Import an item to the store where :
- `<pn>` is the part number of the item (its reference)
- `<name>` is a name or label of the item  

## Add a child to an item :  `add-child <id_parent> <id_child> <quantity>`
Add a child to an item where :
- `<id_parent>`
- `<id_child>`   
- `<quantity>` how many child item to add in the parent item  

# Example

```
buy L289651 chair
buy 305.294.67 table
make room
add-child 3  1  25
add-child 3  2  25
make kitchen
make restaurant
add-child 5  1  80
add-child 5  2  80
make School
add-child 6 3  7
add-child 6 4  1
add-child 6 5  2
```

