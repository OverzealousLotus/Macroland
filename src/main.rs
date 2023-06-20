use macroland::*;

fn main() {
    let btreemap_mac = btreemap!("Hi Ikarite" => true);
    let mut uninit_btreemap = btreemap!(&str, bool);

    let btreeset_mac = btreeset!("Bosnia");
    let mut uninit_btreeset = btreeset!(usize);

    let binary_heap_mac = binary_heap!(100);
    let mut uninit_binary_heap = binary_heap!(usize);

    let hashmap_mac = hashmap!("uwu" => 100);
    let mut uninit_hashmap = hashmap!(usize, String);

    let hashset_mac = hashset!(200, 300, 400);
    let mut uninit_hashset = hashset!(usize);

    let linkedlist_mac = linkedlist!("UWU", "OWO", "U3U");
    let mut uninit_linkedlist = linkedlist!(String);

    let vecdeque_mac = vecdeque!("uwu", "owo", "uwe");
    let mut uninit_vecdeque = vecdeque!(&str);

    uninit_btreemap.insert("Ikarite sussy", false);
    uninit_btreeset.insert(1000);

    uninit_binary_heap.push(10);

    uninit_hashmap.insert(10, String::from("Hi"));
    uninit_hashset.insert(10);

    uninit_linkedlist.push_front(String::from("OWE"));
    uninit_linkedlist.push_back(String::from("YOYOYO"));

    uninit_vecdeque.push_front("Bruh.");
    uninit_vecdeque.push_back("BABABOEEY");

    let boxy = boxed!(10);

    println!(
        "BTreeMap Macro: {:?} | Uninitialized BTreeMap: {:?}\n",
        btreemap_mac, uninit_btreemap
    );
    println!(
        "BTreeSet Macro: {:?} | Uninitialized BTreeSet: {:?}\n",
        btreeset_mac, uninit_btreeset
    );
    println!(
        "BinaryHeap Macro: {:?} | Uninitialized BinaryHeap: {:?}\n",
        binary_heap_mac, uninit_binary_heap
    );
    println!(
        "HashMap Macro: {:?} | Uninitialized HashMap: {:?}\n",
        hashmap_mac, uninit_hashmap
    );
    println!(
        "HashSet Macro: {:?} | Uninitialized HashSet: {:?}\n",
        hashset_mac, uninit_hashset
    );
    println!(
        "LinkedList Macro: {:?} | Uninitialized LinkedList: {:?}\n",
        linkedlist_mac, uninit_linkedlist
    );
    println!(
        "VecDeque Macro: {:?} | Uninitialized VecDeque: {:?}\n",
        vecdeque_mac, uninit_vecdeque
    );
    println!("Box Macro: {:?}", boxy);
}
