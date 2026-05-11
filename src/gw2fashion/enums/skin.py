import enum


class SkinType(enum.Enum):
    AQUABREATHER = ()
    BACKPACK = (True)
    CHEST = (True, True)
    SHOES = (True, True)
    GLOVES = (True)
    HEAD = (True)
    LEGS = (True, True)
    SHOULDERS = (True)
    OUTFIT = (True)
    WEAPON_AQUATIC_A = ()
    WEAPON_AQUATIC_B = ()
    WEAPON_A1 = ()
    WEAPON_A2 = ()
    WEAPON_B1 = ()
    WEAPON_B2 = ()

    def __new__(cls, *args):
        value = len(cls.__members__)
        obj = object.__new__(cls)
        obj._value_ = value
        return obj

    def __init__(self, dyable=False, always_visible=False):
        self.dyable = dyable
        self.always_visible = always_visible

    def api_label(self):
        return SkinAPILabel[self.name]

    def visibility_flag(self):
        return SkinVisibilityFlag[self.name]


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
