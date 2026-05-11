import struct
from dataclasses import dataclass
from typing import Optional

from gw2fashion.enums.skin import SkinType, SkinVisibilityFlag

_SKIN_BYTE_FORMAT = '<H'
_DYEABLE_SKIN_BYTE_FORMAT = '<HHHHH'

@dataclass
class SkinData:
    id: int
    name: str
    visible: bool

    def to_dict(self):
        return {
            'id': self.id,
            'name': self.name,
            'visible': self.visible,
        }


@dataclass
class ColorData:
    id: int
    name: str

    def to_dict(self):
        return {k: v for k, v in self.__dict__.items()}

@dataclass
class DyableSkinData(SkinData):
    dye1: ColorData
    dye2: ColorData
    dye3: ColorData
    dye4: ColorData

    def all_dyes(self):
        return [self.dye1, self.dye2, self.dye3, self.dye4]

    def to_dict(self):
        d = SkinData.to_dict(self)
        d['dye1'] = self.dye1.to_dict()
        d['dye2'] = self.dye2.to_dict()
        d['dye3'] = self.dye3.to_dict()
        d['dye4'] = self.dye4.to_dict()
        return d


class Skin:
    @classmethod
    def from_data(cls, raw_data: dict, skin_type: SkinType):
        skin = raw_data.get('skin', 0)
        return cls(skin_type, skin)
    
    @classmethod
    def unpack_from(cls, skin_type: SkinType, b: bytes, offset: int, visible: bool):
        values = struct.unpack_from(_SKIN_BYTE_FORMAT, b, offset)
        return cls(skin_type, values[0], visible)

    def __init__(self, skin_type: SkinType, skin: int, visible=True, byte_format=_SKIN_BYTE_FORMAT):
        self.skin_type = skin_type
        self.visible = visible or self.skin_type.always_visible
        self.skin = skin
        self.byte_format = byte_format
        self.num_bytes = struct.calcsize(byte_format)
        self.pack_values = [self.skin]

    def __eq__(self, other):
        if isinstance(other, Skin):
            return self.skin_type == other.skin_type and self.visible == other.visible and self.pack_values == other.pack_values
        return False

    def __repr__(self):
        return repr(self.__dict__)

    def to_bytes(self):
        return struct.pack(self.byte_format, *self.pack_values)
    
    def pack_into(self, buffer: bytes, offset: int):
        struct.pack_into(self.byte_format, buffer, offset, *self.pack_values)
    
    def visibility_flag(self):
        if self.visible:
            return self.skin_type.visibility_flag()
        else:
            return SkinVisibilityFlag(0)

    def to_data(self) -> Optional[SkinData]:
        if not self.skin:
            return None
        return SkinData(self.skin, '', self.visible)


class DyableSkin(Skin):
    @classmethod
    def from_data(cls, raw_data: dict, skin_type: SkinType):
        skin = raw_data.get('skin', 0)
        dyes = raw_data.get('dyes', (None, None, None, None))
        return cls(skin_type, skin, dyes)
    
    @classmethod
    def unpack_from(cls, skin_type: SkinType, b: bytes, offset: int, visible: bool):
        values = struct.unpack_from(_DYEABLE_SKIN_BYTE_FORMAT, b, offset)
        return cls(skin_type, values[0], values[1:], visible)

    def __init__(self, skin_type: SkinType, skin: int, dyes: tuple[int, int, int, int], visible=True):
        Skin.__init__(self, skin_type, skin, visible, byte_format=_DYEABLE_SKIN_BYTE_FORMAT)
        self.dyes = dyes
        self.pack_values += (d if d else 1 for d in self.dyes)

    def to_data(self) -> Optional[DyableSkinData]:
        if not self.skin:
            return None
        d1, d2, d3, d4 = self.dyes
        return DyableSkinData(self.skin, '', self.visible, ColorData(d1, ''), ColorData(d2, ''), ColorData(d3, ''), ColorData(d4, ''))


def skin_from_data(skin_type: SkinType, item_data: dict={}):
    skin = item_data.get('skin', 0)
    if skin_type.dyable:
        dyes = item_data.get('dyes', (None, None, None, None))
        return DyableSkin(skin_type, skin, dyes)
    else:
        return Skin(skin_type, skin)

def unpack_skin_from(skin_type: SkinType, b: bytes, offset: int, visible: bool):
    if skin_type.dyable:
        return DyableSkin.unpack_from(skin_type, b, offset, visible)
    else:
        return Skin.unpack_from(skin_type, b, offset, visible)

def skin_type_from_equipment_slot(slot):
    match slot:
        case 'HelmAquatic':
            return SkinType.AQUABREATHER
        case 'Backpack':
            return SkinType.BACKPACK
        case 'Coat':
            return SkinType.CHEST
        case 'Boots':
            return SkinType.SHOES
        case 'Gloves':
            return SkinType.GLOVES
        case 'Helm':
            return SkinType.HEAD
        case 'Leggings':
            return SkinType.LEGS
        case 'Shoulders':
            return SkinType.SHOULDERS
        case 'Outfit':
            return SkinType.OUTFIT
        case 'WeaponAquaticA':
            return SkinType.WEAPON_AQUATIC_A
        case 'WeaponAquaticB':
            return SkinType.WEAPON_AQUATIC_B
        case 'WeaponA1':
            return SkinType.WEAPON_A1
        case 'WeaponA2':
            return SkinType.WEAPON_A2
        case 'WeaponB1':
            return SkinType.WEAPON_B1
        case 'WeaponB2':
            return SkinType.WEAPON_B2
        case 'Accessory1' | 'Accessory2' | 'Ring1' | 'Ring2' | 'Amulet':
            pass
        case _:
            raise ValueError(f'Unkown equipment slot {slot}')
