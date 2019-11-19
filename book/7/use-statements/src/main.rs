// idiomatic way to import internal modules by using absolute paths
use std::collections::HashMap;

// both fmt and io export a Result type. They get used below.
// by namespacing using the parents, we can access both Result types independently.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
