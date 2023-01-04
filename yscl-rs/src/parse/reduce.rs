use super::*;

pub fn reduce_stack(stack: &mut Vec<Unfinished>, top: Node) -> Result<Option<Map>, ()> {
    match stack.last_mut() {
        None => match top {
            Node::Map(top) => Ok(Some(top)),
            _ => Err(()),
        },
        Some(Unfinished::AtomValue(_)) => Err(()),
        Some(Unfinished::List(UnfinishedList { elements })) => {
            elements.push(top);
            Ok(None)
        }
        Some(Unfinished::Map(UnfinishedMap {
            entries,
            pending_entry,
        })) => {
            if pending_entry.has_equal {
                entries.push(MapEntry {
                    key: Identifier::new(pending_entry.key.clone())
                        .expect("Pending key should always be valid"),
                    value: top,
                });

                *pending_entry = UnfinishedMapEntry::empty();

                Ok(None)
            } else {
                Err(())
            }
        }
    }
}
