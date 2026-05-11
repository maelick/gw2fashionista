import enum


class SkinType(enum.Enum):
    AQUABREATHER = 0
    BACKPACK = enum.auto()
    CHEST = enum.auto()
    SHOES = enum.auto()
    GLOVES = enum.auto()
    HEAD = enum.auto()
    LEGS = enum.auto()
    SHOULDERS = enum.auto()
    OUTFIT = enum.auto()
    WEAPON_AQUATIC_A = enum.auto()
    WEAPON_AQUATIC_B = enum.auto()
    WEAPON_A1 = enum.auto()
    WEAPON_A2 = enum.auto()
    WEAPON_B1 = enum.auto()
    WEAPON_B2 = enum.auto()

    def api_label(self):
        return SkinAPILabel[self.name]
    
    def visibility_flag(self):
        return SkinVisibilityFlag[self.name]
    
    def always_visible(self):
        # TODO get rid of match by using custom __new__/__init__: https://docs.python.org/3/howto/enum.html#using-a-custom-new
        match self:
            case SkinType.CHEST | SkinType.LEGS | SkinType.SHOES:
                return True
            case _:
                return False
    
    def is_dyable(self):
        # TODO get rid of match by using custom __new__/__init__: https://docs.python.org/3/howto/enum.html#using-a-custom-new
        match self:
            case SkinType.BACKPACK | SkinType.CHEST | SkinType.SHOES | SkinType.GLOVES | SkinType.LEGS | SkinType.HEAD | SkinType.SHOULDERS | SkinType.OUTFIT:
                return True
            case _:
                return False


class SkinAPILabel(enum.StrEnum):
    AQUABREATHER = 'HelmAquatic'
    BACKPACK = 'Backpack'
    CHEST = 'Coat'
    SHOES = 'Boots'
    GLOVES = 'Gloves'
    HEAD = 'Helm'
    LEGS = 'Leggings'
    SHOULDERS = 'Shoulders'
    OUTFIT = 'Outfit'
    WEAPON_AQUATIC_A = 'WeaponAquaticA'
    WEAPON_AQUATIC_B = 'WeaponAquaticB'
    WEAPON_A1 = 'WeaponA1'
    WEAPON_A2 = 'WeaponA2'
    WEAPON_B1 = 'WeaponB1'
    WEAPON_B2 = 'WeaponB2'

    def skin_type(self):
        return SkinType[self.name]


class SkinVisibilityFlag(enum.Flag):
    AQUABREATHER = enum.auto()
    BACKPACK = enum.auto()
    CHEST = enum.auto()
    SHOES = enum.auto()
    GLOVES = enum.auto()
    HEAD = enum.auto()
    LEGS = enum.auto()
    SHOULDERS = enum.auto()
    OUTFIT = enum.auto()
    WEAPON_AQUATIC_A = enum.auto()
    WEAPON_AQUATIC_B = enum.auto()
    WEAPON_A1 = enum.auto()
    WEAPON_A2 = enum.auto()
    WEAPON_B1 = enum.auto()
    WEAPON_B2 = enum.auto()

    def skin_type(self):
        return SkinType[self.name]
