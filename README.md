# Macroland
-----
Macroland is a simple crate with macros to create various Rust types using
syntactic sugar. Some of these types offered I do not know about myself, but
I have done my best to create a macro for them here. If there are any issues,
feel free to report them to my github.

For example: `hashmap!`, or `binary_heap!`

## Documentation
- See [Macroland on docs.rs](https://docs.rs/macroland/latest/macroland/)

-----
### `hashmap!`

#### Examples
```
// I've tried my best to put things where they belong.
use macroland::hashmap;

let my_hashmap = hashmap!(
    "Key 1" => "Value 1",
    "Key 2" => "Value 2",
    "Key 3" => "Value 3",
);

// Alternatively, you can create an "uninitialized" version.
let mut my_hashmap = hashmap!(&str, usize);
my_hashmap.insert("Hello!", 100);
```

### `hashset!`

#### Examples
```
// I didn't even know HashSets were that different from HashMaps!
use macroland::hashset;

let my_hashset = hashset!(
    1,
    2,
    3,
    4,
    5,
);

// Similar to most others, you can create an "uninitialized" one.
let mut my_hashset = hashset!(usize);
my_hashset.insert(500);
```

### `btreemap!` 

#### Examples
```
// I've used this before, but I am not entirely sure about the difference with it and HashMaps!
use macroland::btreemap;

let my_btreemap = btreemap!(
    "Greetings!" => true,
    "How are you?" => false,
    "I'm good too." => false,
);

// Like the rest.
let mut my_btreemap = btreemap!(&str, bool);
my_btreemap.insert("Hewwo!", false);
```

### `btreeset!`

#### Examples
```
// Here comes the set variant of btree!
use macroland::btreeset;

let my_btreeset = btreeset!(
    "Greetings!",
    "How are you?",
    "I'm good too.",
);

// What's this? Uninit?
let mut my_btreeset = btreeset!(&str);
my_btreeset.insert("Weee!");
```

### `cell!`
```
// Welcome to the wonderful world of celling!
use macroland::cell;

let my_cell = cell!(10);

// Everything works!
my_cell.set(20)

println!("{}", my_cell.get());
```