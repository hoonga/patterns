struct Node<'a, T> {
    item: &'a T,
    tail: Box<Option<Node<'a, T>>>,
}

struct LL<'a, T> {
    head: Option<Node<'a, T>>,
}

impl <'a, T> Iterator for LL<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut node) = self.head {
            let result = node.item;
            self.head = node.tail.take();
            Some(result)
        } else {
            None
        }
    }
}

fn append<'a>(i: &'a String, tail: Node<'a, String>) -> Node<'a, String> {
    Node {
        item: i,
        tail: Box::new(Some(tail))
    }
}

pub fn iterators() {
    let vecs = vec![0,1,2,3];
    let arrays = [0,1,2,3];
    let reference = "1".to_string();
    let lls = LL { 
        head: Some(append(&reference, append(&reference, append(&reference, Node { item: &reference, tail: Box::new(None)})))), 
    };
    let sum1: i32 = vecs.iter().sum();
    let sum2: i32 = arrays.iter().sum();
    let sum3: i32 = lls.into_iter().map(|s| s.parse::<i32>().unwrap()).sum();
    println!("{}, {}, {}", sum1, sum2, sum3);
}