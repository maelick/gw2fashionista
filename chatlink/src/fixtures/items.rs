use crate::{ChatLinkType, fixtures::ChatLinkFixture};

super::fixtures! { $ items, ChatLinkType::Item;
    COPPER_HARVESTING_SICKLE = ("item_copper_harvesting_sickle", "AgH1WQAA"), // 23029
    COPPER_LOGGING_AXE = ("item_copper_logging_axe", "AgH2WQAA"), // 23030
    COPPER_MINING_PICK = ("item_copper_mining_pick", "AgH3WQAA"), // 23031
    BASIC_SALVAGE_KIT = ("item_basic_salvage_kit", "AgEAWgAA"), // 23040
    ZOJJAS_CLAYMORE = ("item_zojjas_claymore", "AgGqtgAA"), // 46762
    ZOJJAS_CLAYMORE_1SIGIL = ("item_zojjas_claymore_with_superior_sigil_of_bloodlust", "AgGqtgBA/18AAA=="), // 46762 + 24575
    ZOJJAS_CLAYMORE_2SIGIL = ("item_zojjas_claymore_with_superior_sigil_of_bloodlust_and_superior_sigil_of_force", "AgGqtgBg/18AACdgAAA="), // 46762 + 24575 + 24615
    ZOJJAS_CLAYMORE_SKINNED = ("item_zojjas_claymore_with_dreamthistle_greatsword_skin", "AgGqtgCAfQ4AAA=="), // 46762 + 3709
    ZOJJAS_CLAYMORE_SKINNED_1SIGIL = ("item_zojjas_claymore_with_dreamthistle_greatsword_skin_and_superior_sigil_of_bloodlust", "AgGqtgDAfQ4AAP9fAAA="), // 46762 + 3709 + 24575
    ZOJJAS_CLAYMORE_SKINNED_2SIGIL = ("item_zojjas_claymore_with_dreamthistle_greatsword_skin,_superior_sigil_of_bloodlust_and_superior_sigil_of_force", "AgGqtgDgfQ4AAP9fAAAnYAAA"), // 46762 + 3709 + 24575 + 24615
}
