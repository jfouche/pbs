# pbs : Product Breakdown Store

Allow you to manage a product breakdown

# Objectives

This software helps to design a solution which integrates multiple items.


# pbs-cli commands

## Add item :  `add <pn> <name>`
Adds a new item to the store where :
- `<pn>` is the part number of the item (its reference)
- `<name>` is a name or label of the item  

# Example

```
add L289651 chair
add 305.294.67 table
add 000001 room
add-child 000001  L289651    25
add-child 000001  305.294.67 25
add 000010 kitchen
add 000020 restaurant
add-child 000020 L289651     80
add-child 000020 305.294.67  80
add 000100 School
add-child 000100 000001 7
add-child 000100 000010 1
add-child 000100 000020 1
```

