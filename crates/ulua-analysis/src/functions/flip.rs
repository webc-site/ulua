use crate::enums::relation::Relation;

pub fn flip(rel: Relation) -> Relation {
  match rel {
    Relation::Subset => Relation::Superset,
    Relation::Superset => Relation::Subset,
    _ => rel,
  }
}
