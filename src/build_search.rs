use std::{
    cmp::{min, Ordering},
    iter,
};

use crate::armor_ron::{Armor, Skill, SKILL_LIMIT_JEWEL_SIZE};

pub type Jewels = [Option<Skill>; 3];

#[derive(Debug)]
pub struct Build {
    pub helmet: Option<(Armor, Jewels)>,
    pub chest: Option<(Armor, Jewels)>,
    pub arm: Option<(Armor, Jewels)>,
    pub waist: Option<(Armor, Jewels)>,
    pub leg: Option<(Armor, Jewels)>,
}

fn optionify_slice_and_add_none<T>(slice: &[T]) -> Vec<Option<&T>> {
    slice
        .iter()
        .map(Some)
        .chain(iter::once(None::<&T>))
        .collect()
}

fn brute_force_search_builds(
    wishes: &[(Skill, u8)],
    helmets: &[Armor],
    chests: &[Armor],
    arms: &[Armor],
    waists: &[Armor],
    legs: &[Armor],
) -> Vec<Build> {
    let helmets = optionify_slice_and_add_none(helmets);
    let chests = optionify_slice_and_add_none(chests);
    let arms = optionify_slice_and_add_none(arms);
    let waists = optionify_slice_and_add_none(waists);
    let legs = optionify_slice_and_add_none(legs);

    let mut builds: Vec<Build> = Vec::with_capacity(500);

    for &helmet in &helmets {
        for &chest in &chests {
            for &arm in &arms {
                for &waist in &waists {
                    for &leg in &legs {
                        let mut delta_wishes: Vec<(Skill, u8)> = wishes.iter().copied().collect();
                        for &option in &[helmet, chest, arm, waist, leg] {
                            if let Some(armor) = option {
                                for &(skill, amount) in &armor.skills {
                                    for (w_skill, w_amount) in delta_wishes.iter_mut() {
                                        if skill == *w_skill {
                                            if amount > *w_amount {
                                                *w_amount = 0;
                                            } else {
                                                *w_amount -= amount;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        //reverse order
                        delta_wishes.sort_unstable_by(|(skill_a, _), (skill_b, _)| {
                            let jewel_size_a =
                                SKILL_LIMIT_JEWEL_SIZE.get(skill_a).unwrap().jewel_size;
                            let jewel_size_b =
                                SKILL_LIMIT_JEWEL_SIZE.get(skill_b).unwrap().jewel_size;
                            match (jewel_size_a, jewel_size_b) {
                                (None, None) => Ordering::Equal,
                                (None, Some(_)) => Ordering::Greater,
                                (Some(_), None) => Ordering::Less,
                                (Some(a), Some(b)) => {
                                    if a > b {
                                        Ordering::Less
                                    } else if a < b {
                                        Ordering::Greater
                                    } else {
                                        Ordering::Equal
                                    }
                                }
                            }
                        });
                        let mut jewel_slots_helmet = extract_slots_copy(helmet);
                        let mut jewel_slots_chest = extract_slots_copy(chest);
                        let mut jewel_slots_arm = extract_slots_copy(arm);
                        let mut jewel_slots_waist = extract_slots_copy(waist);
                        let mut jewel_slots_leg = extract_slots_copy(leg);

                        let mut jewels_helmet: Jewels = [None; 3];
                        let mut index_helmet = 0;
                        let mut jewels_chest: Jewels = [None; 3];
                        let mut index_chest = 0;
                        let mut jewels_arm: Jewels = [None; 3];
                        let mut index_arm = 0;
                        let mut jewels_waist: Jewels = [None; 3];
                        let mut index_waist = 0;
                        let mut jewels_leg: Jewels = [None; 3];
                        let mut index_leg = 0;

                        'zebi: for (skill, amount) in delta_wishes.iter_mut() {
                            if *amount > 0 {
                                for slot in jewel_slots_helmet.iter_mut() {
                                    if let Some(jewel_size) =
                                        SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap().jewel_size
                                    {
                                        if *slot >= jewel_size {
                                            *slot = 0;
                                            *amount -= 1;
                                            jewels_helmet[index_helmet] = Some(*skill);
                                            index_helmet += 1;
                                            if *amount == 0 {
                                                continue 'zebi;
                                            }
                                        }
                                    }
                                }
                            }
                            if *amount > 0 {
                                for slot in jewel_slots_chest.iter_mut() {
                                    if let Some(jewel_size) =
                                        SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap().jewel_size
                                    {
                                        if *slot >= jewel_size {
                                            *slot = 0;
                                            *amount -= 1;
                                            jewels_chest[index_chest] = Some(*skill);
                                            index_chest += 1;
                                            if *amount == 0 {
                                                continue 'zebi;
                                            }
                                        }
                                    }
                                }
                            }
                            if *amount > 0 {
                                for slot in jewel_slots_arm.iter_mut() {
                                    if let Some(jewel_size) =
                                        SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap().jewel_size
                                    {
                                        if *slot >= jewel_size {
                                            *slot = 0;
                                            *amount -= 1;
                                            jewels_arm[index_arm] = Some(*skill);
                                            index_arm += 1;
                                            if *amount == 0 {
                                                continue 'zebi;
                                            }
                                        }
                                    }
                                }
                            }
                            if *amount > 0 {
                                for slot in jewel_slots_waist.iter_mut() {
                                    if let Some(jewel_size) =
                                        SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap().jewel_size
                                    {
                                        if *slot >= jewel_size {
                                            *slot = 0;
                                            *amount -= 1;
                                            jewels_waist[index_waist] = Some(*skill);
                                            index_waist += 1;
                                            if *amount == 0 {
                                                continue 'zebi;
                                            }
                                        }
                                    }
                                }
                            }
                            if *amount > 0 {
                                for slot in jewel_slots_leg.iter_mut() {
                                    if let Some(jewel_size) =
                                        SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap().jewel_size
                                    {
                                        if *slot >= jewel_size {
                                            *slot = 0;
                                            *amount -= 1;
                                            jewels_leg[index_leg] = Some(*skill);
                                            index_leg += 1;
                                            if *amount == 0 {
                                                continue 'zebi;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if delta_wishes.iter().map(|&(_, u8)| u8).sum::<u8>() == 0 {
                            let build = Build {
                                helmet: match helmet {
                                    None => None,
                                    Some(armor) => Some((armor.clone(), jewels_helmet)),
                                },
                                chest: match chest {
                                    None => None,
                                    Some(armor) => Some((armor.clone(), jewels_chest)),
                                },
                                arm: match arm {
                                    None => None,
                                    Some(armor) => Some((armor.clone(), jewels_arm)),
                                },
                                waist: match waist {
                                    None => None,
                                    Some(armor) => Some((armor.clone(), jewels_waist)),
                                },
                                leg: match leg {
                                    None => None,
                                    Some(armor) => Some((armor.clone(), jewels_leg)),
                                },
                            };
                            /*
                            print!("Gagné: ");
                            for &option in &[helmet, chest, arm, waist, leg] {
                                if let Some(armor) = option {
                                    print!("{:?} ", armor.name);
                                } else {
                                    print!("None ");
                                }
                            }
                            println!("");
                            */
                            builds.push(build);
                        }
                    }
                }
            }
        }
    }
    builds
}

fn extract_slots_copy(helmet: Option<&Armor>) -> [u8; 3] {
    match helmet {
        Some(armor) => {
            let mut slots = [0; 3];
            for (key, &slot) in armor.slots.iter().enumerate() {
                slots[key] = slot;
            }
            slots
        }
        None => [0; 3],
    }
}

pub fn pre_selection_then_brute_force_search(wishes: &[(Skill, u8)], helmets: &[Armor], chests: &[Armor], arms: &[Armor], waists: &[Armor], legs: &[Armor]) -> Vec<Build> {
    brute_force_search_builds(
        wishes,
        &search_best_candidates(wishes, helmets),
        &search_best_candidates(wishes, chests),
        &search_best_candidates(wishes, arms),
        &search_best_candidates(wishes, waists),
        &search_best_candidates(wishes, legs),
    )
}



#[derive(Eq, PartialEq, Debug)]
enum OddComparison {
    Better,
    Worse,
    Undefined,
}

fn compare_armors(wishes: &[(Skill, u8)], a: &Armor, b: &Armor) -> OddComparison {
    // add virtual slot depending on the wished skills,
    // if the skill has no jewel, the armor has top priority
    let (delta_skills_a, delta_skills_b) = generate_deltas_skills(&a.skills, &b.skills);
    let (priority_a, virtual_slots_a) = generate_virtual_slots(wishes, &delta_skills_a);
    let (priority_b, virtual_slots_b) = generate_virtual_slots(wishes, &delta_skills_b);

    //println!("{} {:?}", a.name, virtual_slots_a);
    //println!("{} {:?}", b.name, virtual_slots_b);

    let mut slots_a_copy = Vec::with_capacity(3);
    let mut slots_b_copy = Vec::with_capacity(3);

    if a.slots.len() + virtual_slots_a.len() <= b.slots.len() {
        for &s in &[a.slots.as_slice(), virtual_slots_a.as_slice()].concat() {
            slots_a_copy.push(s);
        }
        for &s in &b.slots {
            slots_b_copy.push(s);
        }
        slots_a_copy.sort_unstable();
        slots_b_copy.sort_unstable();
        // if this is the case it means that b has the same slots as
        // a's virtual slots (or better), so b is more flexible than a because
        // we can recreate a's skills by giving the good jewel to b except if a has top priority
        // the (slots_a_copy == slots_b_copy && a.slots.len() < b.slots.len()) means:
        // if real&virtual slots from a are the same as real slots from b
        // and a has less real slots than b, then a is worse than b
        // we can't use the OddComparison::Undefined in this case
        if !priority_a
            && ((slots_a_copy == slots_b_copy && a.slots.len() < b.slots.len())
                || compare_slots(&slots_a_copy, &slots_b_copy) == OddComparison::Worse)
        {
            // println!("{} less than {}", a.name, b.name);
            return OddComparison::Worse;
        }
    } else if b.slots.len() + virtual_slots_b.len() <= a.slots.len() {
        for &s in &a.slots {
            slots_a_copy.push(s);
        }
        for &s in &[b.slots.as_slice(), virtual_slots_b.as_slice()].concat() {
            slots_b_copy.push(s);
        }
        slots_a_copy.sort_unstable();
        slots_b_copy.sort_unstable();
        if !priority_b
            && ((slots_a_copy == slots_b_copy && b.slots.len() < a.slots.len())
                || compare_slots(&slots_a_copy, &slots_b_copy) == OddComparison::Better)
        {
            // println!("{} greater than {}", a.name, b.name);
            return OddComparison::Better;
        }
    }
    /*
    println!(
        "{}: {}, {}: {}",
        a.name,
        a.slots.len() + virtual_slots_a.len(),
        b.name,
        b.slots.len() + virtual_slots_b.len()
    );
    */
    // println!("{} same as {}", a.name, b.name);
    OddComparison::Undefined
}
/**
Garde l'armure que si:

- elle a un des skills
ou elle peut mettre le skill sur un slot

Comparer les armures ainsi obtenues
- dégager les trivialement négligeables: d'autres armures ont plus de slots
cela peut dégager des armures avec les skills choisi mais dominées par
des armures avec plus de slots et qui peut acceuillir le joyau
? -> compter les skills voulus comme des slots virtuels pour mieux comparer

trivial
-> dégager les armures qui n'ont pas le skill et qui n'ont qu'un slot qui ne peuvent pas acceuillir le skill -> étape non obligatoire mais baisse le nombre d'opération de l'étape d'après
-> garder les armures avec le plus de slot, les armures qui ont le skill on un slot virtuel en plus qui a la taille du joyau du skill
*/

/**
Cette phase ne compare pas l'utilité des skills prioritaires
*/

/**
PISTE: l'égalité n'est pas transitive c'est la galewe
*/
fn search_best_candidates(wishes: &[(Skill, u8)], armors: &[Armor]) -> Vec<Armor> {
    let armors: Vec<&Armor> = armors
        .iter()
        .filter(|armor| {
            for (skill, _) in wishes {
                // check if the armor can accept a jewel for one of the wanted skills
                for &slot in &armor.slots {
                    let skill_desc = SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap();
                    if let Some(size) = skill_desc.jewel_size {
                        if slot >= size {
                            return true;
                        }
                    }
                }
                // check if the armor has one of the wanted skills
                for (armor_skill, _) in &armor.skills {
                    if armor_skill == skill {
                        return true;
                    }
                }
            }
            false
        })
        .collect();

    let mut armors_copy = Vec::with_capacity(armors.len());
    for &w in &armors {
        armors_copy.push(w.clone());
    }
    armors_copy.retain(|a| {
        for &b in &armors {
            if compare_armors(wishes, a, b) == OddComparison::Worse {
                // println!("{} worse than {}", a.name, b.name);
                return false;
            }
        }
        true
    });

    for w in &armors_copy {
        dbg!(&w.name);
    }
    println!("");

    armors_copy
}

fn generate_virtual_slots(wishes: &[(Skill, u8)], skills: &[(Skill, u8)]) -> (bool, Vec<u8>) {
    let mut priority = false;
    let mut virtual_slots = Vec::with_capacity(5);
    for (wished_skill, _) in wishes {
        for (skill, amount) in skills {
            if skill == wished_skill {
                let skill_desc = SKILL_LIMIT_JEWEL_SIZE.get(skill).unwrap();
                if let Some(size) = skill_desc.jewel_size {
                    for _ in 0..*amount {
                        virtual_slots.push(size);
                    }
                } else if *amount > 0 {
                    // we check the amount because in a "delta skill" an amount may equal 0
                    priority = true;
                }
            }
        }
    }
    (priority, virtual_slots)
}

/**
accepts only sorted slots slice !!!!!!!!!!
*/
fn compare_slots(slots0: &[u8], slots1: &[u8]) -> OddComparison {
    if slots0.is_empty() {
        if slots1.is_empty() {
            return OddComparison::Undefined;
        }
        return OddComparison::Worse;
    }
    if slots1.is_empty() {
        return OddComparison::Better;
    }
    let mut slot_cmp: Vec<i8> = vec![0; 3];
    for (key, &value) in slots0.iter().enumerate() {
        // +3 because we shift the stuff if there is not enough slots to
        // make the comparison easier
        slot_cmp[key + 3 - slots0.len()] = value as i8;
    }
    for (key, &value) in slots1.iter().enumerate() {
        slot_cmp[key + 3 - slots1.len()] -= value as i8;
    }
    let mut sign = 0;
    for i in slot_cmp {
        if i != 0 {
            if sign == 0 {
                sign = i;
            } else if sign.signum() != i.signum() {
                // if the signs are different it means that
                // the two armors has not very comparable slots
                // like 1,1,1 and 0,0,3
                // => 1-0, 1-0, 1- 3 = different signs
                // or 2,2,2 and 1,3,3
                // having 3 2-sized slots can have a different use
                // than having 2 3-sized slots and 1 1-sized slot
                return OddComparison::Undefined;
            }
        }
    }
    if sign == 0 {
        OddComparison::Undefined
    } else if sign.signum() == -1 {
        OddComparison::Worse
    } else {
        OddComparison::Better
    }
}

/**
When comparing two armors with the same skills, we can "remove" them to make the comparison easier
*/
fn generate_deltas_skills(
    skills0: &[(Skill, u8)],
    skills1: &[(Skill, u8)],
) -> (Vec<(Skill, u8)>, Vec<(Skill, u8)>) {
    let mut delta0 = Vec::with_capacity(skills0.len());
    let mut delta1 = Vec::with_capacity(skills1.len());

    for &s in skills0 {
        delta0.push(s);
    }
    for &s in skills1 {
        delta1.push(s);
    }

    let mut to_do0 = Vec::with_capacity(3);
    let mut to_do1 = Vec::with_capacity(3);

    for (key0, &(skill0, amount0)) in delta0.iter().enumerate() {
        for (key1, &(skill1, amount1)) in delta1.iter().enumerate() {
            if skill0 == skill1 {
                let delta = min(amount0, amount1);
                //to please the borrow checker
                to_do0.push((key0, (skill0, amount0 - delta)));
                to_do1.push((key1, (skill0, amount1 - delta)));
            }
        }
    }

    for &(key, couple) in &to_do0 {
        delta0[key] = couple;
    }
    for &(key, couple) in &to_do1 {
        delta1[key] = couple;
    }

    (delta0, delta1)
}
