use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet};

use macroland::*;

fn main() {
    let hashmap_mac = hashmap!("uwu" => 100);
    let mut uninit_hashmap = hashmap!(usize, String);

    let hashset_mac = hashset!(200, 300, 400);
    let mut uninit_hashset = hashset!(usize);

    let btreemap_mac = btreemap!("Hi Ikarite" => true);
    let mut uninit_btreemap = btreemap!(&str, bool);

    let btreeset_mac = btreeset!("Bosnia");
    let mut uninit_btreeset = btreeset!(usize);

    let binary_heap_mac = binary_heap!(100);
    let mut uninit_binary_heap = binary_heap!(usize);

    uninit_hashmap.insert(10, String::from("Hi"));
    uninit_hashset.insert(10);

    uninit_btreemap.insert("Ikarite sussy", false);
    uninit_btreeset.insert(1000);

    uninit_binary_heap.push(10);

    let boxy = boxed!(10);

    println!(
        "HashMap Macro: {:?} | Uninitialized HashMap: {:?}",
        hashmap_mac, uninit_hashmap
    );
    println!(
        "HashSet Macro: {:?} | Uninitialized HashSet: {:?}",
        hashset_mac, uninit_hashset
    );
    println!(
        "BTreeMap Macro: {:?} | Uninitialized BTreeMap: {:?}",
        btreemap_mac, uninit_btreemap
    );
    println!(
        "BTreeSet Macro: {:?} | Uninitialized BTreeSet: {:?}",
        btreeset_mac, uninit_btreeset
    );
    println!(
        "BinaryHeap Macro: {:?} | Uninitialized BinaryHeap: {:?}",
        binary_heap_mac, uninit_binary_heap
    );
    println!("Box Macro: {:?}", boxy);
}
