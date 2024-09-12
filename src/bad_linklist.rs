// With this design, the content and reference are separated,
// to prevent the first element allocating on the stack while heap for others,
// causing inconsistency.

// This stores the content
struct ListContent{
    element: i32,
    next: ListNode
}

// This stores the reference,
// and to telling the List Content which can either be the next node or being empty
enum ListNode{
    Nothing,
    Content(Box<ListContent>)
}

// We also need a struct to locate the first reference of the list
pub struct BadLinkedList{
    head: ListNode
}

use std::mem;

impl BadLinkedList{
    // When we create an initial node, we need to insert a null reference,
    // so we have inserted the head with "Nothing", selected from the enum which represents a null pointer.
    // This is technically a constructor in other languages.
    pub fn new() -> Self {
        BadLinkedList{head: ListNode::Nothing}
    }

    pub fn push(&mut self, input: i32){
        let content = Box::new(ListContent{
            element: input,
            // this is a hack for replace a original memory with a new value
            // in this case, the nothing node replaces the self.head,
            // while returning the original un-replaced value from self.head.
            //
            // In this example, since we have moved head reference,
            // suggesting the reference is mutable, so we need to use &mut at input
            next: mem::replace(&mut self.head, ListNode::Nothing)
        });

        // Since we have taken the self for other use,
        // we must giving some back to the original reference.
        // Rust works like trading which have to return something to the owner,
        // after taking some stuff from them.
        self.head = ListNode::Content(content);
    }

    // in this case, since the function can either return something or "nothing",
    // we need to return that as an optional
    pub fn pop(&mut self) -> Option<i32>{

        // without defining all the case in the enum,
        // the complier can't identify the type of the variable
        let result;

        match mem::replace(&mut self.head, ListNode::Nothing) {
            ListNode::Nothing => {
                result = None;
            }
            ListNode::Content(content) => {
                self.head = content.next;
                result = Some(content.element);
            }
        };
        
        result
    }
}

mod bad_linklist_test{
    use super::BadLinkedList;

    #[test]
    fn basics(){
        let mut bad_linked_list = BadLinkedList::new();

        assert_eq!(bad_linked_list.pop(), None);

        bad_linked_list.push(69);
        bad_linked_list.push(420);
        bad_linked_list.push(314);

        // tests:
        assert_eq!(bad_linked_list.pop(), Some(314));
        assert_eq!(bad_linked_list.pop(), Some(420));

    }
}