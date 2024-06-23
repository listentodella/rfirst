use std::rc::Rc;

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Node {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());

    // get_downstream()得到的是不可变引用
    // 即便node6定义为mut,编译器也不会放过它,因为Rc不具备内部可变性
    // 如果一定想改,就得借助能够提供 内部可变性, 比如 RefCell
    // let node5 = Node::new(5);
    // let mut node6 = node1.get_downstream().unwrap();
    // node6.update_downstream(Rc::new(node5));

    println!("node1:{:?}, node2:{:?}", node1, node2);
}
