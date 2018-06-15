mod viae;

fn main() {
    let vrs = viae::get_viae();

    let start_urbe = "Roma".to_string();
    // let start_via = find_via(&vrs, start_urbe.clone()).expect("Starting urbe not found in our viae!");
    // println!("START: {} [via: {}]", start_urbe, start_via);

    //let dest_urbe = "Ariminium".to_string();
    let dest_urbe = "Parma".to_string();
    // let dest_via = find_via(&vrs, dest_urbe.clone()).expect("Destination urbe not found un our database");
    // println!("DESTINATION: {} [via: {}]", dest_urbe, dest_via);

    //let mut path : Vec<String> = vec![];
    find_path(&vrs, start_urbe, dest_urbe, "".to_string(), 0, 0);
}

// TODO: accept &str instead of string, but lifetimes need to be defined - one I understand them
// fn find_via(vrs: &Vec<viae::Via>, nomen: String) -> Option<(String)> {
//     for via in vrs { // "via" is a borrowed reference
//         for urbe in &via.urbes { // "urbe" is a borrowed reference
//             if urbe.nomen == nomen {
//                 return Some( via.nomen.clone() );
//                 //println!("{} [{}]", via.nomen, urbe.miliarium);
//             }
//         }
//     }
//     None
// }

fn find_path(vrs: &Vec<viae::Via>, start_urbe: String, dest_urbe: String, from_via: String, steps: usize, depth: usize) {
    if depth > 10 {
        println!("TOO MUCH RECURSION");
        return;
    }
    // Find all via which traverse the urbe
    // TODO: maybe use a grep() to find the via
    for via in vrs {
        // Avoid walking the same via we are coming from, or it would be an endless loop
        if via.nomen == from_via {
            continue;
        }
        let via_len = *(&via.urbes.len());
        for i in 0 .. via_len { // "urbe" is a borrowed reference
            let urbe = &via.urbes[i];
            if urbe.nomen == start_urbe {
                for _dp in 0 .. depth { print!("  "); }
                println!("VIA: {} [{} - {}]", via.nomen, urbe.nomen, urbe.miliarium);
                // Walk the via in both directions
                // TODO: use slices and iterate on them with an iterator
                if i < via_len-1 {
                    let mut cursteps = steps;
                    for j in i+1 .. via_len {
                        let next_urbe = &via.urbes[j];
                        cursteps += 1;
                        for _dp in 0 .. depth { print!("  "); }
                        println!("STEP {} - NX: {} [{} - {}]", cursteps, via.nomen, next_urbe.nomen, next_urbe.miliarium);
                        if next_urbe.nomen == dest_urbe {
                            println!("FOUND DEST!!!\n");
                            break;
                        } else {
                            find_path(vrs, next_urbe.nomen.clone(), dest_urbe.clone(), via.nomen.clone(), cursteps, depth+1);
                        }
                    }
                }
                if i > 0 {
                    let mut cursteps = steps;
                    for j in (0 .. i).rev() {
                        let prev_urbe = &via.urbes[j];
                        cursteps += 1;
                        for _dp in 0 .. depth { print!("  "); }
                        println!("STEP {} - PV: {} [{} - {}]", cursteps, via.nomen.clone(), prev_urbe.nomen, prev_urbe.miliarium);
                        if via.nomen == dest_urbe {
                            println!("FOUND DEST!!!\n");
                            break;
                        } else {
                            find_path(vrs, prev_urbe.nomen.clone(), dest_urbe.clone(), via.nomen.clone(), cursteps, depth+1);
                        }
                    }
                }
                //Vec.index()
                //return Some( via.nomen.clone() );
            }
        }
    }
    // if path.len() < 10 {
    //     path.push( start_urbe.clone() );
    //     find_path(start_urbe, start_via, path);
    // }
}