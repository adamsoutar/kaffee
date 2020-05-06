use std::collections::HashMap;
use crate::interpretting::interpreter_utils::*;

// I think this is a tracing garbage collector,
// although I haven't really done any research or seen other implementations,
// I'm just diving in.

pub fn gc_collect(
    alloced: &mut HashMap<usize, AllocedValue>,
    scopestack: &mut Vec<HashMap<String, usize>>) {

    // No VISPLANE_OVERFLOWs here :P
    let mut visplane: Vec<usize> = vec![];

    // TODO: Follow object references
    //       until then, this could leave dangling refs
    for scope in scopestack {
        for (_, idx) in scope {
            visplane.push(*idx)
        }
    }

    // Clean up everything we can no longer see
    let all_idx: Vec<usize> = alloced.keys().map(|x| x.clone()).collect();
    for ai in all_idx {
        if !visplane.contains(&ai) {
            alloced.remove(&ai);
        }
    }
}
