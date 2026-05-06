import struct

from gw2fashion.enums.skin import SkinType, SkinVisibilityFlag

_SKIN_BYTE_FORMAT = '<H'
_DYEABLE_SKIN_BYTE_FORMAT = '<HHHHH'


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
        self.visible = visible or self.skin_type.always_visible()
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


def skin_from_data(skin_type: SkinType, item_data: dict={}):
    skin = item_data.get('skin', 0)
    if skin_type.is_dyable():
        dyes = item_data.get('dyes', (None, None, None, None))
        return DyableSkin(skin_type, skin, dyes)
    else:
        return Skin(skin_type, skin)


def unpack_skin_from(skin_type: SkinType, b: bytes, offset: int, visible: bool):
    if skin_type.is_dyable():
        return DyableSkin.unpack_from(skin_type, b, offset, visible)
    else:
        return Skin.unpack_from(skin_type, b, offset, visible)
