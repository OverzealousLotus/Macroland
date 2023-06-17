//! # Macroland
//!
//! Macroland is a Rust Crate, which provides macros for shorthand definitions
//! of various types.

/// `binary_heap` creates a Binary Heap. This can be done in two different ways.
///
/// # Examples
///
/// ```
/// use std::collections::BinaryHeap;
///
/// use macroland::binary_heap;
///
/// let mut uninit_binary_heap = binary_heap!(usize);
/// uninit_binary_heap.push(100);
/// // Or
/// let my_binary_heap: BinaryHeap<usize> = binary_heap!(100);
///
/// assert_eq!(uninit_binary_heap, my_binary_heap);
/// ```
#[macro_export]
macro_rules! binary_heap {
    ($value:ty) => {
        {
            let new_binary_heap: BinaryHeap<$value> = BinaryHeap::new();
            new_binary_heap
        }
    };

    ( $($value:expr),* ) => {
        {
            let mut new_binary_heap = BinaryHeap::new();
            $( new_binary_heap.push($value); )*
            new_binary_heap
        }
    };
}

/// `btreeset!` creates a BTreeSet. This can be done in two different ways.
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
            let new_btreeset: BTreeSet<$value> = BTreeSet::new();
            new_btreeset
        }
    };

    ( $($value:expr),*) => {
        {
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
            let new_btreemap: BTreeMap<$key, $value> = BTreeMap::new();
            new_btreemap
        }
    };

    ($($key:expr => $value:expr),*) => {
        {
            let mut new_btreemap = BTreeMap::new();
            $( new_btreemap.insert($key, $value); )*
            new_btreemap
        }
    }
}

/// `hashset!` creates a HashSet. This can be done in two different ways.
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
            let new_hashset: HashSet<$value> = HashSet::new();
            new_hashset
        }
    };
    ($($value:expr),*) => {
        {
            let mut new_hashset = HashSet::new();
            $( new_hashset.insert($value); )*
            new_hashset
        }
    };
}

/// `hashmap!` creates a HashMap. This can be done in two different ways.
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
            let new_map: HashMap<$key, $value> = HashMap::new();
            new_map
        }
    };
    ($($key:expr => $value:expr),*) => {
        {
            let mut new_map = HashMap::new();
            $( new_map.insert($key, $value); )*
            new_map
        }
    };
}

/// `boxed!` provides a simple way to box a given type.
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
