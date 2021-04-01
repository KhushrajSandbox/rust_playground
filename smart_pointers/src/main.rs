enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let b = List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(
                3,
                Box::new(List::Cons(
                    4,
                    Box::new(List::Cons(
                        5, 
                        Box::new(List::Cons(
                            6, 
                            Box::new(List::Nil)
                        ))
                    )),
                )),
            )),
        )),
    );
}
