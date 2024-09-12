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
pub struct LinkedList{
    head: ListNode
}

use std::mem;

impl LinkedList{
    // When we create an initial node, we need to insert a null reference,
    // so we have inserted the head with "Nothing", selected from the enum which represents a null pointer.
    // This is technically a constructor in other languages.
    pub fn new() -> Self {
        LinkedList{head: ListNode::Nothing}
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
}


