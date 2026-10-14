use std::collections::{HashMap, HashSet};

use gw2lib::model::{
    authenticated::characters::{Equip, EquipmentTab, Slot as EquipmentSlot},
    items::ItemId,
};

use gw2fashionista_chatlink::templates::{
    FashionSlot,
    appearance::{Appearance, DyeId, Dyes, SkinId},
    wardrobe::{WardrobeSlot, WardrobeTemplate},
};

#[derive(Clone, Debug)]
pub struct Equipment {
    pub char_name: String,
    pub tab_id: usize,
    pub tab_name: String,
    pub slots: Vec<Equip>,
}

impl Equipment {
    pub fn new(char_name: &str, api_data: &EquipmentTab) -> Self {
        Equipment {
            char_name: char_name.to_string(),
            tab_id: api_data.tab,
            tab_name: api_data.name.clone(),
            slots: api_data.equipment.clone(),
        }
    }

    pub fn with_slots(&self, slots: Vec<Equip>) -> Self {
        Equipment {
            char_name: self.char_name.clone(),
            tab_id: self.tab_id,
            tab_name: self.tab_name.clone(),
            slots,
        }
    }

    pub fn all_item_ids(&self) -> HashSet<ItemId> {
        HashSet::from_iter(
            self.slots
                .iter()
                .filter_map(|s| if s.skin.is_none() { Some(s.id) } else { None }),
        )
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(level = "warn", skip_all, fields(char_name = %self.char_name, tab_id = self.tab_id)))]
    pub fn to_template(&self) -> WardrobeTemplate {
        let mut slots = HashMap::new();
        for equip in &self.slots {
            if let Some(slot) = equip.slot.as_ref().and_then(to_wardrobe_slot) {
                slots.insert(slot, appearance_from(&slot, equip));
            }
        }
        WardrobeTemplate::new(slots)
    }
}

fn to_wardrobe_slot(slot: &EquipmentSlot) -> Option<WardrobeSlot> {
    match slot {
        EquipmentSlot::HelmAquatic => Some(WardrobeSlot::Aquabreather),
        EquipmentSlot::Backpack => Some(WardrobeSlot::Backpack),
        EquipmentSlot::Coat => Some(WardrobeSlot::Chest),
        EquipmentSlot::Boots => Some(WardrobeSlot::Shoes),
        EquipmentSlot::Gloves => Some(WardrobeSlot::Gloves),
        EquipmentSlot::Helm => Some(WardrobeSlot::Head),
        EquipmentSlot::Leggings => Some(WardrobeSlot::Legs),
        EquipmentSlot::Shoulders => Some(WardrobeSlot::Shoulders),
        EquipmentSlot::WeaponAquaticA => Some(WardrobeSlot::WeaponAquaticA),
        EquipmentSlot::WeaponAquaticB => Some(WardrobeSlot::WeaponAquaticB),
        EquipmentSlot::WeaponA1 => Some(WardrobeSlot::WeaponA1),
        EquipmentSlot::WeaponA2 => Some(WardrobeSlot::WeaponA2),
        EquipmentSlot::WeaponB1 => Some(WardrobeSlot::WeaponB1),
        EquipmentSlot::WeaponB2 => Some(WardrobeSlot::WeaponB2),
        _ => None,
    }
}

#[cfg_attr(feature = "tracing", tracing::instrument(level = "warn", skip_all, fields(?slot)))]
fn appearance_from(slot: &WardrobeSlot, equip: &Equip) -> Appearance {
    let skin = template_skin(equip).unwrap_or_default();
    if slot.dyeable() {
        Appearance::Dyeable {
            skin,
            visible: true,
            dyes: template_dyes(equip).unwrap_or_default(),
        }
    } else {
        Appearance::NonDyeable {
            skin,
            visible: true,
        }
    }
}

fn template_skin(equip: &Equip) -> Option<SkinId> {
    equip
        .skin
        .and_then(|id| {
            u16::try_from(id)
                .inspect_err(|_| {
                    #[cfg(feature = "tracing")]
                    tracing::warn!(
                        message = "skin id does not fit in a wardrobe template, leaving slot empty",
                        skin_id = id
                    )
                })
                .ok()
        })
        .map(Into::into)
}

fn template_dyes(equip: &Equip) -> Option<Dyes> {
    equip
        .dyes
        .as_ref()
        .map(|dyes| dyes_from_iter(dyes.iter().map(DyeId::from)))
}

fn dyes_from_iter(mut dyes: impl Iterator<Item = DyeId>) -> Dyes {
    Dyes::new(
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
        dyes.next().unwrap_or_default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use gw2lib::model::authenticated::characters::Location;
    use rstest::rstest;

    #[rstest]
    #[case::non_dyable_no_data(WardrobeSlot::WeaponA1, None, None, Appearance::NonDyeable { skin: 0.into(), visible: true })]
    #[case::dyable_no_data(WardrobeSlot::Head, None, None, Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Default::default() })]
    #[case::non_dyable_with_dyes(WardrobeSlot::WeaponA1, None, Some(vec![Some(1), Some(2), Some(3), Some(4)]), Appearance::NonDyeable { skin: 0.into(), visible: true })]
    #[case::dyable_with_empty_dyes(WardrobeSlot::Head, None, Some(vec![]), Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Dyes::default() })]
    #[case::dyable_with_one_dye(WardrobeSlot::Head, None, Some(vec![Some(42)]), Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Dyes::new(42.into(), 1.into(), 1.into(), 1.into()) })]
    #[case::dyable_with_missing_dyes(WardrobeSlot::Head, None, Some(vec![None, Some(2)]), Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Dyes::new(1.into(), 2.into(), 1.into(), 1.into()) })]
    #[case::dyable_with_five_dyes(WardrobeSlot::Head, None, Some(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]), Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Dyes::new(1.into(), 2.into(), 3.into(), 4.into()) })]
    #[case::non_dyable_with_id(WardrobeSlot::WeaponA1, Some(5), None, Appearance::NonDyeable { skin: 5.into(), visible: true })]
    #[case::dyable_with_id(WardrobeSlot::Head, Some(5), Some(vec![Some(1), Some(2), Some(3), Some(4)]), Appearance::Dyeable { skin: 5.into(), visible: true, dyes: Dyes::new(1.into(), 2.into(), 3.into(), 4.into()) })]
    #[case::non_dyable_with_max_id(WardrobeSlot::WeaponA1, Some(u16::MAX as u32), None, Appearance::NonDyeable { skin: u16::MAX.into(), visible: true })]
    #[case::dyable_with_max_id(WardrobeSlot::Head, Some(u16::MAX as u32), Some(vec![Some(1), Some(2), Some(3), Some(4)]), Appearance::Dyeable { skin: u16::MAX.into(), visible: true, dyes: Dyes::new(1.into(), 2.into(), 3.into(), 4.into()) })]
    #[case::non_dyable_with_id_overflow(WardrobeSlot::WeaponA1, Some(u16::MAX as u32 + 1), None, Appearance::NonDyeable { skin: 0.into(), visible: true })]
    #[case::dyable_with_id_overflow(WardrobeSlot::Head, Some(u16::MAX as u32 + 1), Some(vec![Some(1), Some(2), Some(3), Some(4)]), Appearance::Dyeable { skin: 0.into(), visible: true, dyes: Dyes::new(1.into(), 2.into(), 3.into(), 4.into()) })]
    fn test_appearance_from(
        #[case] slot: WardrobeSlot,
        #[case] skin_id: Option<u32>,
        #[case] dyes: Option<Vec<Option<u16>>>,
        #[case] expected: Appearance,
    ) {
        let equip = Equip {
            id: 123,
            slot: None,
            count: None,
            infusions: None,
            upgrades: None,
            skin: skin_id,
            stats: None,
            binding: None,
            location: Location::Equipped,
            charges: None,
            bound_to: None,
            dyes,
            tabs: None,
        };

        let actual = appearance_from(&slot, &equip);
        assert_eq!(actual, expected);
    }
}
