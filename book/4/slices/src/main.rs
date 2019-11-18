fn main() {
    let string = String::from("testing first word");
    let first = first_word_index(&string);
    println!("first_word(\"{}\") returns len {}", string, first);

    // danger, this is broken.
    // what happens if string got emptied?
    let first_w = &string[0..first].trim();
    println!("first word is {}", first_w);

    let first_w = first_word(&string);
    println!("first_word(\"{}\") returned {}", string, first_w);

    // making sure no breakage appears by using &str as argument type
    let first_w_slice = first_word_slice(&string[..]);
    println!("first_word_slice(\"{}\") returned {}", string, first_w_slice);

    // many value types can be sliced:
    let a = ["1", "2", "3", "4", "5"];

    let slice = &a[1..3];
    println!("array slices: {}", slice.join(" "));

    // more about vec and array slices in chapter 8
}

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let i = first_word_index(&s);

    &s[..i].trim()
}

fn first_word_index_slice(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let i = first_word_index_slice(&s);

    &s[..i].trim()    
}
