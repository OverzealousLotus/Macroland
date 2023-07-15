//! # Macroland
//!
//! Macroland is a Rust Crate, which provides macros for shorthand definitions
//! of various types. Think how `vec!` is used to create empty, or filled up vectors.
//! Macroland is full of various macros for this exact purpose.

/// `binary_heap` creates a Binary Heap. You can either create an empty variant, with
/// its type explicity given, or a list of values.
///
/// # Examples
///
/// ```
/// use macroland::binary_heap;
///
/// let mut uninit_binary_heap = binary_heap!(usize);
/// uninit_binary_heap.push(100);
/// // Or
/// let my_binary_heap: BinaryHeap<usize> = binary_heap!(100, 200, 300);
///
/// assert_eq!(uninit_binary_heap, my_binary_heap);
/// ```
#[macro_export]
macro_rules! binary_heap {
    ($value:ty) => {
        {
            use std::collections::BinaryHeap;

            let new_binary_heap: BinaryHeap<$value> = BinaryHeap::new();
            new_binary_heap
        }
    };

    ( $($value:expr),* ) => {
        {
            use std::collections::BinaryHeap;

            let mut new_binary_heap = BinaryHeap::new();
            $( new_binary_heap.push($value); )*

            new_binary_heap
        }
    };
}

/// `btreeset!` creates a BTreeSet. Like HashSet, you can predefine its members, or the
/// type of its future members.
///
/// # Examples
///
/// ```
/// use macroland::btreeset;
///
/// let mut uninit_btreeset = btreeset!(&str);
/// uninit_btreeset.insert("Hello!");
/// // Or
/// let my_btreeset = btreeset!("Hello!");
///
/// assert_eq!(uninit_btreeset, my_btreeset);
/// ```
#[macro_export]
macro_rules! btreeset {
    ($value:ty) => {
        {
            use std::collections::BTreeSet;

            let new_btreeset: BTreeSet<$value> = BTreeSet::new();
            new_btreeset
        }
    };

    ( $($value:expr),*) => {
        {
            use std::collections::BTreeSet;

            let mut new_btreeset = BTreeSet::new();
            $( new_btreeset.insert($value); )*

            new_btreeset
        }
    };
}

/// `btreemap!` creates a BTreeMap. This can be done in two different ways.
///
/// # Examples
///
/// ```
/// use macroland::btreemap;
///
/// let mut uninit_btreemap = btreemap!(&str, usize);
/// uninit_btreemap.insert("Uno", 1);
/// uninit_btreemap.insert("Dos", 2);
/// uninit_btreemap.insert("Tres", 3);
/// // Or
/// let my_btreemap = btreemap!(
///     "Uno" => 1,
///     "Dos" => 2,
///     "Tres" => 3
/// );
///
/// assert_eq!(uninit_btreemap, my_btreemap);
/// ```
#[macro_export]
macro_rules! btreemap {
    ($key:ty, $value:ty) => {
        {
            use std::collections::BTreeMap;

            let new_btreemap: BTreeMap<$key, $value> = BTreeMap::new();
            new_btreemap
        }
    };

    ($($key:expr => $value:expr),*) => {
        {
            use std::collections::BTreeMap;

            let mut new_btreemap = BTreeMap::new();
            $( new_btreemap.insert($key, $value); )*

            new_btreemap
        }
    }
}

/// `boxed!` provides a simple way to box a given type, without using ::new() each
/// time you want to.
///
/// # Example
///
/// ```
/// use macroland::boxed;
///
/// let macroland_box = boxed!(Some(100));
/// let normal_box = Box::new(Some(100));
///
/// assert_eq!(macroland_box, normal_box);
/// ```
#[macro_export]
macro_rules! boxed {
    ($value:expr) => {{
        Box::new($value)
    }};
}

/// `hashset!` creates a HashSet. You can either define its members during the macro call,
/// or create an empty HashSet with its type predefined.
///
/// # Examples
///
/// ```
/// use macroland::hashset;
///
/// let mut uninit_hashset = hashset!(&str);
/// uninit_hashset.insert("Hello!");
/// // Or
/// let my_hashset = hashset!("Hello!");
///
/// assert_eq!(uninit_hashset, my_hashset);
/// ```
#[macro_export]
macro_rules! hashset {
    ($value:ty) => {
        {
            use std::collections::HashSet;

            let new_hashset: HashSet<$value> = HashSet::new();
            new_hashset
        }
    };
    ($($value:expr),*) => {
        {
            use std::collections::HashSet;

            let mut new_hashset = HashSet::new();
            $( new_hashset.insert($value); )*

            new_hashset
        }
    };
}

/// `hashmap!` creates a HashMap. The OG is back, y'all... I'm kidding. I heard about there
/// being a builtin `hashmap!` macro not too long ago, I guess I wasn't around then. So,
/// this one is for all those who remember.
///
/// # Examples
///
/// ```
/// use macroland::hashmap;
///
/// let mut uninit_hashmap = hashmap!(&str, usize);
/// uninit_hashmap.insert("Uno", 1);
/// uninit_hashmap.insert("Dos", 2);
/// uninit_hashmap.insert("Tres", 3);
/// // Or
/// let my_hashmap = hashmap!(
///     "Uno" => 1,
///     "Dos" => 2,
///     "Tres" => 3
/// );
///
/// assert_eq!(uninit_hashmap, my_hashmap);
/// ```
#[macro_export]
macro_rules! hashmap {
    ($key:ty, $value:ty) => {
        {
            use std::collections::HashMap;

            let new_map: HashMap<$key, $value> = HashMap::new();
            new_map
        }
    };
    ($($key:expr => $value:expr),*) => {
        {
            use std::collections::HashMap;

            let mut new_map = HashMap::new();
            $( new_map.insert($key, $value); )*

            new_map
        }
    };
}

/// `linkedlist!` creates a LinkedList. Alike to the `vecdeque!` macro, all members are pushed
/// to the front of the list, when predefined.
///
/// # Examples
///
/// ```
/// use macroland::linkedlist;
///
/// let mut uninit_linkedlist = linkedlist!(isize);
/// uninit_linkedlist.push_front(10);
/// uninit_linkedlist.push_back(20);
/// // Or
/// let my_linkedlist = linkedlist!(
///     10,
///     20,
///     30,
/// );
/// ```
#[macro_export]
macro_rules! linkedlist {
    ($value:ty) => {
        {
            use std::collections::LinkedList;

            let new_linkedlist: LinkedList<$value> = LinkedList::new();
            new_linkedlist
        }
    };
    ( $($value:expr),* ) => {
        {
            use std::collections::LinkedList;

            let mut new_linkedlist = LinkedList::new();
            $( new_linkedlist.push_front($value); )*

            new_linkedlist
        }
    };
}

/// `vecdeque!` creates a VecDeque. Be warned, members are pushed into the *front* of the
/// VecDeque<T> when predefining the members using this macro.
///
/// # Examples
///
/// ```
/// use macroland::vecdeque;
///
/// let mut uninit_vecdeque = vecdeque!(&str);
/// uninit_vecdeque.push_front("I'm going ahead!");
/// uninit_vecdeque.push_back("I'm going behind!");
/// // Or
/// let my_vecdeque = vecdeque!(
///     "I'm always going ahead!",
///     "No matter how many members are pushed!",
///     "Uwu",
/// );
/// ```
#[macro_export]
macro_rules! vecdeque {
    ($value:ty) => {{
        use std::collections::VecDeque;

        let new_vecdeque: VecDeque<$value> = VecDeque::new();
        new_vecdeque
    }};
    ( $($value:expr),* ) => {
        {
            use std::collections::VecDeque;

            let mut new_vecdeque = VecDeque::new();
            $(new_vecdeque.push_front($value); )*

            new_vecdeque
        }
    };
}
