# HashMap

```rust
use std::collections::HashMap;

fn main(){
    //create hashmap
    //can omit explicit type declaraton if later insert is used from where type can be deduced
    let mut map = HashMap::new();

    //modify
    map.insert(1,2);

    //get a value => will return optional
    let x = 1;
    map.get(&x);

    //iterate over all values
    for (key,val) in &map{
        println!("key = {} and val = {}",key,val);
    }

    //normal insert will overwrite the previous value
    //inserting when there is no value for that key
    map.entry(&x).or_insert(50);
}
```