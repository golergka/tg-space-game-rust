use super::*;

#[derive(Identifiable, Queryable, Debug)]
pub struct StarLink {
    pub id: i32,
    pub a_id: i32,
    pub a_obj_type: GalaxyObjectType,
    pub b_id: i32,
    pub b_obj_type: GalaxyObjectType,
}

#[derive(Insertable, Debug)]
#[table_name = "star_links"]
pub struct NewStarLink {
    pub a_id: i32,
    pub a_obj_type: GalaxyObjectType,
    pub b_id: i32,
    pub b_obj_type: GalaxyObjectType,
}

use std::collections::hash_map::DefaultHasher;

impl NewStarLink {
    pub fn new(a: &GalaxyObject, b: &GalaxyObject) -> NewStarLink {
        NewStarLink {
            a_id: a.id,
            a_obj_type: a.obj_type,
            b_id: b.id,
            b_obj_type: b.obj_type
        }
    }

    // TODO make these methods a trait and implement this trait for both NewStarLink and StarLink
    pub fn side_a(&self) -> GalaxyObject {
        GalaxyObject {
            id: self.a_id,
            obj_type: self.a_obj_type
        }
    }

    pub fn side_b(&self) -> GalaxyObject {
        GalaxyObject {
            id: self.b_id,
            obj_type: self.b_obj_type
        }
    }
}

use std::cmp;

impl PartialEq for NewStarLink {
    fn eq(&self, other: &NewStarLink) -> bool {
        (
            self.side_a() == other.side_a() &&
            self.side_b() == other.side_b()
        ) ||
        (
            self.side_a() == other.side_b() &&
            self.side_b() == other.side_a()
        )
    }
}

use std::hash::{Hash, Hasher};

impl Hash for NewStarLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Create side hashers
        let mut hasher_a = DefaultHasher::new();
        let mut hasher_b = DefaultHasher::new();
        // Hash sides
        self.side_a().hash(&mut hasher_a);
        self.side_b().hash(&mut hasher_b);
        // Finish side hashers
        let hash_a = hasher_a.finish();
        let hash_b = hasher_b.finish();
        // Order side hashes
        let hash_max = cmp::max(hash_a, hash_b);
        let hash_min = cmp::min(hash_a, hash_b);
        // Hash in order
        hash_max.hash(state);
        hash_min.hash(state);
    }
}

impl Eq for NewStarLink {}

use rand::distributions::{Distribution, Weighted, WeightedChoice};
use rand::Rng;

pub fn generate_links<R: Rng>(
    mut elements: &mut [Weighted<GalaxyObject>],
    link_amount: usize,
    unique: bool,
    mut rng: R,
) -> Vec<NewStarLink> 
{
    let mut result: Vec<NewStarLink> = Vec::new();
    rng.shuffle(elements);

    // Required links, so that graph is linked
    let min_links = elements.len() - 1;
    for i in 0..min_links {
        result.push(NewStarLink::new(&elements[i].item, &elements[i + 1].item));
    }

    // Extra links
    let max_links = elements.len() * (elements.len() - 1) / 2;
    let mut links_left = cmp::max(link_amount, max_links) - min_links;
    let mut attempts = links_left * links_left;

    let wc = WeightedChoice::new(&mut elements);

    while links_left > 0 && attempts > 0 {
        let side_a = wc.sample(&mut rng);
        let side_b = wc.sample(&mut rng);
        if side_a != side_b {
            let link = NewStarLink::new(&side_a, &side_b);
            if !unique || !result.contains(&link) {
                result.push(link);
                links_left -= 1;
            }
        }
        attempts -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn generate_links_creates_required_links() {
        let rng = StepRng::new(0, 1);
        let item1 = GalaxyObject {
            id: 1,
            obj_type: GalaxyObjectType::Sector
        };
        let item2 = GalaxyObject {
            id: 2,
            obj_type: GalaxyObjectType::Sector
        };

        let mut elements: [Weighted<GalaxyObject>; 2] = [
            Weighted::<GalaxyObject>{
                weight: 1,
                item: item1.clone()
            },
            Weighted::<GalaxyObject>{
                weight: 1,
                item: item2.clone()
            }
        ];
        let result = generate_links(&mut elements, 0usize, false, rng);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], NewStarLink::new(&item1, &item2));
    }
}