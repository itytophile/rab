use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    array,
    cmp::{min, Ordering},
    collections::HashMap,
    iter,
};

use crate::armor_and_skills::{Armor, Gender, Skill};

pub type Jewels = [Option<Skill>; 3];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub helmet: Option<(Armor, Jewels)>,
    pub chest: Option<(Armor, Jewels)>,
    pub arm: Option<(Armor, Jewels)>,
    pub waist: Option<(Armor, Jewels)>,
    pub leg: Option<(Armor, Jewels)>,
    pub talisman: Option<(Armor, Jewels)>,
    pub weapon_jewels: Jewels,
}

impl Build {
    pub fn get_all_skills_and_amounts(&self) -> HashMap<Skill, u8> {
        let mut hm = HashMap::with_capacity(5);
        for opt in array::IntoIter::new([
            &self.helmet,
            &self.chest,
            &self.arm,
            &self.waist,
            &self.leg,
            &self.talisman,
        ])
        .flatten()
        {
            let (armor, jewels) = opt;
            for (skill, amount) in armor.skills.iter() {
                if !hm.contains_key(skill) {
                    hm.insert(*skill, 0);
                }
                *hm.get_mut(skill).unwrap() += amount
            }
            for jewel in jewels.iter().flatten() {
                if !hm.contains_key(jewel) {
                    hm.insert(*jewel, 0);
                }
                *hm.get_mut(jewel).unwrap() += 1
            }
        }
        for jewel in self.weapon_jewels.iter().flatten() {
            if !hm.contains_key(jewel) {
                hm.insert(*jewel, 0);
            }
            *hm.get_mut(jewel).unwrap() += 1
        }
        hm
    }
}

/// Returns a Vec containing every item of the provided slice but wrapped in an Option, then add None
/// at the end of the Vec.
/// # Why
/// The main goal is to test every possible combination of armor pieces including the "empty" armor piece.
/// The None represents this "empty" armor piece and if we need None, then every other pieces must be "optionified".
fn optionify_slice_and_add_none<T>(slice: &[T]) -> Vec<Option<&T>> {
    slice
        .iter()
        .map(Some)
        .chain(iter::once(None::<&T>))
        .collect()
}

/// Naive search of compatible builds. This is not recommended to call this function directly (see [`pre_selection_then_brute_force_search`]).
/// It will try every possible build combination with the slices provided (thanks to a cartesian product).
/// Then it will try to set jewels on the builds if the wishes are not yet fulfilled.
/// # Detailed steps
/// For each combination:
/// 1. Remove from the wishes the combination's skills
/// 2. Try to fulfill the remaining wishes with jewels without spoiling slots, if not possible the algorithm stops
/// 3. Remove the repetitive found builds: If a build has the "same" armor pieces than another build
/// but the first build has more empty armor pieces then we can ignore the second build. (Example in code)
fn brute_force_search_builds(
    wishes: &[(Skill, u8)],
    all_armor_slices: AllArmorSlices,
    weapon_slots: [u8; 3],
) -> Vec<Build> {
    let AllArmorSlices {
        helmets,
        chests,
        arms,
        waists,
        legs,
        talismans,
    } = all_armor_slices;
    let mut builds: Vec<Build> = Vec::with_capacity(500);

    for v in array::IntoIter::new([helmets, chests, arms, waists, legs, talismans])
        .map(optionify_slice_and_add_none)
        .multi_cartesian_product()
    {
        let (helmet, chest, arm, waist, leg, talisman) = (v[0], v[1], v[2], v[3], v[4], v[5]);

        // We remove from the wishes the skills that are already present
        // in the armor. Then we know what are the jewels to set.
        let mut delta_wishes: Vec<(Skill, u8)> = wishes.iter().copied().collect();
        for &option in &[helmet, chest, arm, waist, leg, talisman] {
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
        // reverse order sort
        // We will place the bigger jewels first to be sure to not
        // spoil big slots with little jewels
        delta_wishes.sort_unstable_by(|(skill_a, _), (skill_b, _)| {
            let jewel_size_a = skill_a.get_jewel_size();
            let jewel_size_b = skill_b.get_jewel_size();
            match (jewel_size_a, jewel_size_b) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(a), Some(b)) => b.cmp(&a),
            }
        });

        const NB_PARTS: usize = 7;

        let mut possible_jewels_for_each_part = [Jewels::default(); NB_PARTS];
        let mut jewel_indices = [0; NB_PARTS];
        let mut empty_armor_slots = [
            extract_slots_copy(&helmet),
            extract_slots_copy(&chest),
            extract_slots_copy(&arm),
            extract_slots_copy(&waist),
            extract_slots_copy(&leg),
            extract_slots_copy(&talisman),
            weapon_slots,
        ];

        'wish_loop: for (skill, amount) in delta_wishes.iter_mut() {
            for (jewel_slots, (jewels, index)) in empty_armor_slots.iter_mut().zip(
                possible_jewels_for_each_part
                    .iter_mut()
                    .zip(jewel_indices.iter_mut()),
            ) {
                if *amount > 0 {
                    for slot in jewel_slots.iter_mut() {
                        if let Some(jewel_size) = skill.get_jewel_size() {
                            if *slot >= jewel_size {
                                *slot = 0;
                                *amount -= 1;
                                jewels[*index] = Some(*skill);
                                *index += 1;
                                if *amount == 0 {
                                    // the wish is satisfied so we can
                                    // skip to the next wish
                                    continue 'wish_loop;
                                }
                            }
                        } else {
                            // We have a skill without a jewel (like Critical Boost)
                            // rip in peace
                            break 'wish_loop;
                        }
                    }
                }
            }
        }

        if delta_wishes.iter().all(|(_, amount)| *amount == 0) {
            let build = Build {
                helmet: helmet.map(|armor| (armor.clone(), possible_jewels_for_each_part[0])),
                chest: chest.map(|armor| (armor.clone(), possible_jewels_for_each_part[1])),
                arm: arm.map(|armor| (armor.clone(), possible_jewels_for_each_part[2])),
                waist: waist.map(|armor| (armor.clone(), possible_jewels_for_each_part[3])),
                leg: leg.map(|armor| (armor.clone(), possible_jewels_for_each_part[4])),
                talisman: talisman.map(|armor| (armor.clone(), possible_jewels_for_each_part[5])),
                weapon_jewels: possible_jewels_for_each_part[6],
            };

            // Avoid having redondant builds like:
            // A B C D E
            // and
            // A B None D E
            // If the build with the None works, then it's useless to keep the first build
            // Naive algorithm ahead

            let mut push_it = true;
            let mut replacement: Option<&mut Build> = None;
            // maybe less than needed, maybe overkill. I don't really know
            let mut to_remove = Vec::with_capacity(20);

            for (key, old_build) in builds.iter_mut().enumerate() {
                let mut old_has_better_none = false;
                let mut new_has_better_none = false;
                // don't want to use .iter() because it will give &&Option<>
                // and don't want to use [build.helmet, build.chest, ...].iter() because it will copy
                // the elements (they don't even implement the Copy trait)
                for couple in array::IntoIter::new([
                    &build.helmet,
                    &build.chest,
                    &build.arm,
                    &build.waist,
                    &build.leg,
                    &build.talisman,
                ])
                .zip([
                    &old_build.helmet,
                    &old_build.chest,
                    &old_build.arm,
                    &old_build.waist,
                    &old_build.leg,
                    &old_build.talisman,
                ]) {
                    match couple {
                        (None, Some(_)) => {
                            new_has_better_none = true;
                            if old_has_better_none {
                                // if they have None at different places
                                break;
                            }
                        }
                        (Some(_), None) => {
                            old_has_better_none = true;
                            if new_has_better_none {
                                break;
                            }
                        }
                        (Some(part), Some(old_part)) if part.0 != old_part.0 => {
                            // they are not comparable
                            old_has_better_none = new_has_better_none;
                            break;
                        }
                        _ => {}
                    };
                }

                if old_has_better_none != new_has_better_none {
                    push_it = false;
                    if old_has_better_none {
                        break;
                    }
                    // have to move this reference
                    // out of the loop to please the
                    // compiler
                    if replacement.is_none() {
                        replacement = Some(old_build);
                    } else {
                        // We continue to search builds worse
                        // than the new part (yes this is possible)
                        to_remove.push(key);
                    }
                }
            }
            if let Some(place) = replacement {
                *place = build;
            } else if push_it {
                builds.push(build);
            }
            to_remove.sort_unstable();

            // thank you https://stackoverflow.com/a/57948703
            for index in to_remove.drain(..).rev() {
                builds.swap_remove(index);
            }
        }
    }

    builds
}

pub struct AllArmorSlices<'a> {
    pub helmets: &'a [Armor],
    pub chests: &'a [Armor],
    pub arms: &'a [Armor],
    pub waists: &'a [Armor],
    pub legs: &'a [Armor],
    pub talismans: &'a [Armor],
}

/// Makes a copy of the armor slots. Useful when we want to write in the array.
fn extract_slots_copy(helmet: &Option<&Armor>) -> [u8; 3] {
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

/// Calls [`search_best_candidates`] on each slice before calling [`brute_force_search_builds`]. Returns
/// the results of the brute force search. This is the recommended function to call when we want to search builds.
pub fn pre_selection_then_brute_force_search(
    wishes: &[(Skill, u8)],
    all_armor_slices: AllArmorSlices,
    gender: Gender,
    weapon_slots: [u8; 3],
) -> Vec<Build> {
    let AllArmorSlices {
        helmets,
        chests,
        arms,
        waists,
        legs,
        talismans,
    } = all_armor_slices;
    brute_force_search_builds(
        wishes,
        AllArmorSlices {
            helmets: &search_best_candidates(wishes, helmets, gender),
            chests: &search_best_candidates(wishes, chests, gender),
            arms: &search_best_candidates(wishes, arms, gender),
            waists: &search_best_candidates(wishes, waists, gender),
            legs: &search_best_candidates(wishes, legs, gender),
            talismans: &search_best_candidates(wishes, talismans, gender),
        },
        weapon_slots,
    )
}

/// When we are comparing armor pieces we can't really say that an armor is "equal" to another.
/// Like a piece with two 1-sized slots and another with one 3-sized slot. We can't really say
/// that the first piece is better than the other and vice-versa and we can't say that they are equal either.
///
/// As an example, let's say that we are comparing three pieces:
/// - piece A with two 1-sized slots
/// - piece B with one 3-sized slots
/// - piece C with one 1-sized slot and one 2-sized slot
///
/// Let's say that if we can't say which piece is the best between two pieces then the two pieces are equal.
///
/// Then A == B and B == C. But if this is true, is A == C? No, because C is clearly better than A. This is why
/// this is "strange" to use directly [`Ordering`] with [`Ordering::Equal`].
#[derive(Eq, PartialEq, Debug)]
enum OddComparison {
    Better,
    Worse,
    Undefined,
}

/// The most difficult function in my opinion (ityt). Compares two armor pieces and returns an [`OddComparison`].
/// # Detailed steps
/// 1. Generate a "delta" skill for each piece. See [`generate_deltas_skills`].
/// 2. Generate virtual slots for each piece. See [`generate_virtual_slots`].
/// 3. We compare the pieces. Some details in code.
///
/// Special comparison: If A can recreate B's skills with jewels and A has still equal or better
/// free slots than B's then A is better. This can change because jewels are expensive and sometimes
/// we just want the piece that have already the skills.
fn compare_armors(wishes: &[(Skill, u8)], a: &Armor, b: &Armor) -> OddComparison {
    // add virtual slot depending on the wished skills,
    // if the skill has no jewel (like Critical Boost), the armor has top priority
    let (delta_skills_a, delta_skills_b) = generate_deltas_skills(&a.skills, &b.skills);
    let (priority_a, virtual_slots_a) = generate_virtual_slots(wishes, &delta_skills_a);
    let (priority_b, virtual_slots_b) = generate_virtual_slots(wishes, &delta_skills_b);

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
            return OddComparison::Better;
        }
    }

    OddComparison::Undefined
}

/// Finds the best pieces to fulfill the wishes.
/// # Detailed steps
/// 1. Remove the pieces that are not compatible with the hunter's gender.
/// 2. Remove the pieces that have not the wished skills and can't accept a jewel for these skills.
/// 3. Compare the selected pieces with [`compare_armors`] and remove the pieces that are worse than another piece.
fn search_best_candidates(wishes: &[(Skill, u8)], armors: &[Armor], gender: Gender) -> Vec<Armor> {
    // trivial sort
    let armors: Vec<&Armor> = armors
        .iter()
        .filter(|armor| armor.gender == Gender::Neutral || armor.gender == gender)
        .filter(|armor| {
            for (skill, _) in wishes {
                // check if the armor can accept a jewel for one of the wanted skills
                for &slot in &armor.slots {
                    if let Some(size) = skill.get_jewel_size() {
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
    // non trivial sort
    armors_copy.retain(|a| {
        for &b in &armors {
            if compare_armors(wishes, a, b) == OddComparison::Worse {
                return false;
            }
        }
        true
    });

    armors_copy
}

/// Creates dummy slots from skills' jewel size. Returns (true, _) if one of the skill doesn't
/// have a jewel.
/// # Why
/// This is useful when we want to know if a piece with only its slots and jewels can be better or equal than
/// another with its true skills and slots.
fn generate_virtual_slots(wishes: &[(Skill, u8)], skills: &[(Skill, u8)]) -> (bool, Vec<u8>) {
    let mut priority = false;
    let mut virtual_slots = Vec::with_capacity(5);
    for (wished_skill, _) in wishes {
        for (skill, amount) in skills {
            if skill == wished_skill {
                if let Some(size) = skill.get_jewel_size() {
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

/// Compares two pieces' slot and returns an [`OddComparison`].
/// See [`OddComparison`] for details.
///
/// # Warning
/// This functions works only if the provided slices are sorted.
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

/// Compares skills of each piece and removes identical skills from them.
/// # Why
/// When we are comparing two armor pieces, it becomes easier when we remove
/// the same skills to only compare the true differences.
///
/// As an example: let's say with have A with the skills [(Botanist,2),(CriticalBoost,1)]
/// and B with [(Botanist,1),(CriticalBoost,1),(AttackBoost,1)]
///
/// the delta skills will be:
/// - for A: [(Botanist,1),(CriticalBoost,0)]
/// - for B: [(Botanist,0),(CriticalBoost,0),(AttackBoost,1)]
fn generate_deltas_skills(skills0: &[(Skill, u8)], skills1: &[(Skill, u8)]) -> DeltasSkills {
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

type DeltasSkills = (Vec<(Skill, u8)>, Vec<(Skill, u8)>);
