use macroland::*;

fn btree_macros() {
    let btreemap_mac = btreemap!("Hi Ikarite" => true);
    let mut uninit_btreemap = btreemap!(&str, bool);

    let btreeset_mac = btreeset!("Bosnia");
    let mut uninit_btreeset = btreeset!(usize);

    uninit_btreemap.insert("Ikarite sussy", false);
    uninit_btreeset.insert(1000);

    println!(
        "BTreeMap Macro: {:?} | Uninitialized BTreeMap: {:?}\n",
        btreemap_mac, uninit_btreemap
    );
    println!(
        "BTreeSet Macro: {:?} | Uninitialized BTreeSet: {:?}\n",
        btreeset_mac, uninit_btreeset
    );
}

fn cell_macros() {
    let cell_mac = cell!([1, 2, 3]);

    let refcell_mac = refcell!([4, 5, 6]);

    let uninit_oncecell = oncecell!(String);

    uninit_oncecell.set(String::from("Meow!")).expect("Error");

    println!("Cell Macro: {:?}\n", cell_mac.get());
    println!("RefCell Macro: {:?}\n", refcell_mac.borrow());
    println!(
        "Uninitialized OnceCell: {:?}\n",
        uninit_oncecell.get().unwrap()
    );
}

fn hash_macros() {
    let hashmap_mac = hashmap!("uwu" => 100);
    let mut uninit_hashmap = hashmap!(usize, String);

    let hashset_mac = hashset!(200, 300, 400);
    let mut uninit_hashset = hashset!(usize);

    uninit_hashmap.insert(10, String::from("Hi"));
    uninit_hashset.insert(10);

    println!(
        "HashMap Macro: {:?} | Uninitialized HashMap: {:?}\n",
        hashmap_mac, uninit_hashmap
    );
    println!(
        "HashSet Macro: {:?} | Uninitialized HashSet: {:?}\n",
        hashset_mac, uninit_hashset
    );
}

fn misc_macros() {
    let boxy = boxed!(10);

    println!("Box Macro: {:?}\n", boxy);
}

fn other_collection_macros() {
    let binary_heap_mac = binary_heap!(100);
    let mut uninit_binary_heap = binary_heap!(usize);

    let linkedlist_mac = linkedlist!("UWU", "OWO", "U3U");
    let mut uninit_linkedlist = linkedlist!(String);

    let vecdeque_mac = vecdeque!("uwu", "owo", "uwe");
    let mut uninit_vecdeque = vecdeque!(&str);

    uninit_binary_heap.push(10);

    uninit_linkedlist.push_front(String::from("OWE"));
    uninit_linkedlist.push_back(String::from("YOYOYO"));

    uninit_vecdeque.push_front("Bruh.");
    uninit_vecdeque.push_back("BABABOEEY");

    println!(
        "BinaryHeap Macro: {:?} | Uninitialized BinaryHeap: {:?}\n",
        binary_heap_mac, uninit_binary_heap
    );
    println!(
        "LinkedList Macro: {:?} | Uninitialized LinkedList: {:?}\n",
        linkedlist_mac, uninit_linkedlist
    );
    println!(
        "VecDeque Macro: {:?} | Uninitialized VecDeque: {:?}\n",
        vecdeque_mac, uninit_vecdeque
    );
}

fn reference_macros() {
    let rc_mac = rc!("Mew?");

    println!("Rc Macro: {:?}", rc_mac);
}
fn main() {
    btree_macros();
    cell_macros();
    hash_macros();
    misc_macros();
    other_collection_macros();
    reference_macros();
}
