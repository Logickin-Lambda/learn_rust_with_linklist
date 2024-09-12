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
struct LinkedList{
    head: ListNode
}

impl LinkedList{
    // When we create an initial node, we need to insert a null reference,
    // so we have inserted the head with "Nothing", selected from the enum which represents a null pointer.
    // This is technically a constructor in other languages.
    pub fn new() -> Self {
        LinkedList{head: ListNode::Nothing}
    }
}


