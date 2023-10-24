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

## Create a new item :  `item make <name>`
Create a new item in the store where :
- `<name>` is a name or label of the item  
A part number will be auto-generated

## Buy an existing item :  `item buy <pn> <name>`
Import an item to the store where :
- `<pn>` is the part number of the item (its reference)
- `<name>` is a name or label of the item  

## Add a child to an item :  `child add <id_parent> <id_child> <quantity>`
Add a child to an item where :
- `<id_parent>`
- `<id_child>`   
- `<quantity>` how many child item to add in the parent item  

## Remove a child from an item :  `child del <id_parent> <id_child>`
Remove a child from an item where :
- `<id_parent>`
- `<id_child>`   

# Example

```
item buy L289651 chair
item buy 305.294.67 table
item make room
child-add 3  1  25
child add 3  2  25
item make kitchen
item make restaurant
child add 5  1  80
child add 5  2  80
item make School
child add 6 3  7
child add 6 4  1
child add 6 5  2
tree 6
```



