#![allow(unused)]

use ink_engine_runtime::InkListItem;

fn main() {
    //let json = "".to_owned();
    //let story = ink_engine_runtime::Story::new_from_json(json);

    //let j = InkListItem::from_full_name("part1.part2").unwrap();
    let j = InkListItem::new("part1", "part2");

    println!("{}", j.full_name());
    println!("{}", j.item_name());
    println!("{}", j.origin_name());
}
