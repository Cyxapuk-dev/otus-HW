use std::rc::Rc;
use std::cell::RefCell;

pub struct Node<T> {
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { value, next: None }))
    }
}

pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
    pub tail: Option<Rc<RefCell<Node<T>>>>,
    pub length: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new(value);
        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
        } else {
            self.head = Some(Rc::clone(&new_node));
        }
        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn insert_after(&mut self, n: usize, value: T) -> Result<(), String> {
        if n >= self.length {
            return Err(format!("Index out of bounds: {}", n));
        }

        let new_node = Node::new(value);
        let mut current = Rc::clone(self.head.as_ref().unwrap());

        for _ in 0..n {
            let next_node = current.borrow().next.clone();
            if let Some(node) = next_node {
                current = node;
            } else {
                // This case should ideally not be reached if n < self.length
                return Err("Unexpected end of list during traversal.".to_string());
            }
        }

        let old_next = current.borrow_mut().next.take();
        current.borrow_mut().next = Some(Rc::clone(&new_node));
        new_node.borrow_mut().next = old_next;

        if current.borrow().next.is_none() {
            // If the new node is inserted at the end, update the tail
            self.tail = Some(Rc::clone(&new_node));
        }
        self.length += 1;
        Ok(())
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LinkedListIterator<T> {
    current: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Iterator for LinkedListIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.current.take();
        if let Some(node) = current_node {
            self.current = node.borrow().next.clone();
            Some(node)
        } else {
            None
        }
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = Rc<RefCell<Node<T>>>;
    type IntoIter = LinkedListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIterator { current: self.head }
    }
}

fn main() {
    println!("Demonstrating LinkedList functionality:");

    // 1. Create a new LinkedList and push elements
    let mut list = LinkedList::new();
    list.push_back(10);
    list.push_back(20);
    list.push_back(30);
    println!("\nAfter pushing 10, 20, 30:");
    print_list(&list);

    // 2. Insert an element after a specific index
    match list.insert_after(1, 25) {
        Ok(_) => println!("\nAfter inserting 25 after index 1:"),
        Err(e) => println!("Error inserting: {}", e),
    }
    print_list(&list);

    // 3. Insert at the end (after the last element)
    match list.insert_after(list.length - 1, 35) {
        Ok(_) => println!("\nAfter inserting 35 after the last element:"),
        Err(e) => println!("Error inserting: {}", e),
    }
    print_list(&list);

    // 4. Attempt to insert out of bounds
    match list.insert_after(10, 100) {
        Ok(_) => println!("Successfully inserted out of bounds (should not happen)."),
        Err(e) => println!("\nAttempted to insert out of bounds (index 10): {}", e),
    }

    // 5. Demonstrate iteration
    println!("\nIterating through the list:");
    for (i, node) in list.into_iter().enumerate() {
        println!("Element at index {}: {}", i, node.borrow().value);
    }
}

fn print_list<T: std::fmt::Display>(list: &LinkedList<T>) {
    print!("List (length {}): ", list.length);
    let mut current = list.head.as_ref().map(Rc::clone);
    while let Some(node_rc) = current {
        print!("{} ", node_rc.borrow().value);
        current = node_rc.borrow().next.as_ref().map(Rc::clone);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_linked_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert_eq!(list.length, 1);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 1);

        list.push_back(2);
        assert_eq!(list.length, 2);
        assert_eq!(list.head.as_ref().unwrap().borrow().value, 1);
        assert_eq!(list.tail.as_ref().unwrap().borrow().value, 2);
        assert_eq!(
            list.head
                .as_ref()
                .unwrap()
                .borrow()
                .next
                .as_ref()
                .unwrap()
                .borrow()
                .value,
            2
        );
    }

    #[test]
    fn test_iterator() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next().unwrap().borrow().value, 1);
        assert_eq!(iter.next().unwrap().borrow().value, 2);
        assert_eq!(iter.next().unwrap().borrow().value, 3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_insert_after() {
        // Insert in the middle
        let mut list1 = LinkedList::new();
        list1.push_back(1);
        list1.push_back(2);
        list1.push_back(3);
        list1.insert_after(0, 10).unwrap(); // Insert 10 after 1 (index 0)
        assert_eq!(list1.length, 4);
        let mut values = Vec::new();
        for node in list1.into_iter() {
            values.push(node.borrow().value);
        }
        assert_eq!(values, vec![1, 10, 2, 3]);

        // Insert at the end
        let mut list2 = LinkedList::new();
        list2.push_back(1);
        list2.push_back(2);
        list2.push_back(3);
        list2.insert_after(2, 20).unwrap(); // Insert 20 after 3 (index 2)
        assert_eq!(list2.length, 4);
        let mut values = Vec::new();
        for node in list2.into_iter() {
            values.push(node.borrow().value);
        }
        assert_eq!(values, vec![1, 2, 3, 20]);

        // Test out of bounds
        let mut list3 = LinkedList::new();
        list3.push_back(1);
        let result = list3.insert_after(5, 30);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Index out of bounds: 5");
    }
}