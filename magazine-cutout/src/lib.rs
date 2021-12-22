use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut notes: HashMap<&str, usize> = HashMap::new();
    let mut magazines: HashMap<&str, usize> = HashMap::new();
    for n in note.iter() {
        notes.entry(n).and_modify(|c| { *c += 1 }).or_insert(1);
    }

    for n in magazine.iter() {
        magazines.entry(n).and_modify(|c| { *c += 1 }).or_insert(1);
    }

    for (nk, nv) in notes.iter() {
        if let Some(mv) = magazines.get(*nk) {
            if nv > mv { return false; }
        } else { return false; }
    }

    true
}

/// awesome version
pub fn can_construct_note_awesome(magazine: &[&str], note: &[&str]) -> bool {
    magazine.iter().fold(0, |acc, &word| acc + note.contains(&word) as usize) >= note.len()
}
