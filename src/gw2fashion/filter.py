from gw2fashion.enums.skin import SkinFlag

ALL = SkinFlag(2 ** len(SkinFlag)-1)

WEAPONS = (
    SkinFlag.WEAPON_AQUATIC_A |
    SkinFlag.WEAPON_AQUATIC_B |
    SkinFlag.WEAPON_A1 |
    SkinFlag.WEAPON_A2 |
    SkinFlag.WEAPON_B1 |
    SkinFlag.WEAPON_B2
)

ARMORS = (
    SkinFlag.AQUABREATHER |
    SkinFlag.CHEST |
    SkinFlag.SHOES |
    SkinFlag.GLOVES |
    SkinFlag.HEAD |
    SkinFlag.LEGS |
    SkinFlag.SHOULDERS
)

UNDERWATER = (
    SkinFlag.AQUABREATHER |
    SkinFlag.WEAPON_AQUATIC_A |
    SkinFlag.WEAPON_AQUATIC_B
)


class SkinFilter:
    skins: SkinFlag

    def __init__(self, skins=ALL):
        self.skins = skins

    def add(self, skin: SkinFlag):
        self.skins |= skin

    def remove(self, skin: SkinFlag):
        self.skins = self.skins ^ skin & self.skins
    
    def filter(self, skin: SkinFlag):
        skins &= skin

    def invert(self):
        self.skins = ~self.skins

    def no_weapons(self):
        self.remove(WEAPONS)

    def no_armor(self):
        self.remove(ARMORS)

    def no_backpack(self):
        self.remove(SkinFlag.BACKPACK)

    def no_outfit(self):
        self.remove(SkinFlag.OUTFIT)

    def no_underwater(self):
        self.remove(UNDERWATER)

    def only_underwater(self):
        self.filter(UNDERWATER)

    def visible_skins(self):
        return self.skins.skin_types()
    
    def hidden_skins(self):
        return (~self.skins).skin_types()
