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

    def flag(self):
        return SkinFlag[self.name]


class SkinFlag(enum.Flag):
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

    def skin_types(self):
        return (f.skin_type() for f in self)


# Ensure that names for visibility flags and skin types are the same
assert set(f.name for f in SkinFlag) == set(t.name for t in SkinType)
