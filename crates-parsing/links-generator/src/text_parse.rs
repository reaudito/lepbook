use comrak::{
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};

pub fn text_parse_fn(document: &str) {
    let arena = Arena::new();
    let root = parse_document(&arena, document, &ComrakOptions::default());

    let mut list_number = vec![];
    let mut current_number = 0;

    fn walk_node<'a>(
        node: &'a AstNode<'a>,
        list_number: &mut Vec<usize>,
        current_number: &mut usize,
    ) {
        match &node.data.borrow().value {
            NodeValue::Heading(_) => {
                // Reset the numbering for new sections
                list_number.clear();
                *current_number = 0;
            }
            NodeValue::List(_) => {
                // For each list, start numbering from 1
                *current_number = 0;
            }
            NodeValue::Item(_) => {
                // Increment the list number
                *current_number += 1;
                list_number.push(*current_number);
            }
            NodeValue::Link(link) => {
                // Print the current hierarchical numbering and the link text/URL
                let number_string: String =
                    list_number.iter().map(|n| n.to_string() + ".").collect();
                let text = node
                    .descendants()
                    .find_map(|n| {
                        if let NodeValue::Text(ref text) = n.data.borrow().value {
                            Some(text.clone())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                println!("{} {}: {:?}", number_string, text, link.url);
            }
            _ => {}
        }

        // Recursively process children
        for child in node.children() {
            walk_node(child, list_number, current_number);
        }

        if matches!(node.data.borrow().value, NodeValue::Item(_)) {
            list_number.pop();
        }
    }

    walk_node(root, &mut list_number, &mut current_number);
}
