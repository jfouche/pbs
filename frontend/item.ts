export interface  Item {
    id: number,
    pn: string,
    name: string
  }

  export interface ArrayOfItem extends Array<Item>{}
