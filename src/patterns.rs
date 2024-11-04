enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn append(i:i32, l: List) -> List {
    List::Cons(i, Box::new(l))
}

pub fn all_sorts_of_patterns() {
    // matching
    let compounded = Some((1,2,None::<i32>));
    match compounded {
        None => println!("NONE"),
        Some((1, y, Some(_))) => println!("{}", y),
        Some((_, _, Some(_))) => println!("Something plz"),
        Some((1, 1, None)) => println!("one one none"),
        Some((x, _, None)) => println!("GOAL {}", x),
    }
    
    // if let expression
    let maybe_value = Some(Some(None::<String>));
    let value = if let Some(x) = maybe_value {
        if let Some(y) = x {
            if let Some(z) = y {
                z
            } else {
                "We got nothing in depth 3".to_string()
            }
        } else {
            "We got nothing in depth 2".to_string()
        }
    } else {
        "We got nothing in depth 1".to_string()
    };
    println!("{}", value);

    // while let
    let list_of_five = append(0, append(1, append(2, append(3, append(4, List::Nil)))));
    let mut l = list_of_five;
    while let List::Cons(head, tail) = l {
        l = *tail;
        println!("{}",head);
    }

    // for (and iter)
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // ownership and patterns
    let x = Some(Box::new(vec![1,2,3]));
    if let Some(mut y) = x {
        println!("{}", (*y).pop().unwrap());
    }
    // println!("{}", x.unwrap().pop().unwrap());
}