use std::collections::BTreeSet;

pub type SeenSet = BTreeSet<(*const (), *const ())>;
