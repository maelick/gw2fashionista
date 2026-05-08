import struct
import base64
from typing import Self

from gw2fashion.enums.skin import SkinAPILabel, SkinType, SkinVisibilityFlag
from gw2fashion.skins import Skin, DyableSkin, skin_from_data, unpack_skin_from
from gw2fashion.enums.chatlink import ChatLinkType

_HEADER_BYTE_FORMAT = '<B'
_VISIBILITY_BYTE_FORMAT = '<H'


class FashionTemplate:
    @classmethod
    def from_data(cls, equipment_tab: list[dict]):
        # TODO outfit missing from API data
        skins = {}
        for item in equipment_tab:
            if item['slot'] in list(SkinAPILabel):
                skin_type = SkinAPILabel(item['slot']).skin_type()
                skin = skin_from_data(skin_type, item)
                skins[skin.skin_type] = skin
        return cls(skins)
    
    @classmethod
    def from_bytes(cls, b):
        if len(b) != 97:
            raise ValueError("Fashion template link should be 97 bytes long")
        if b[0] != ChatLinkType.WARDROBE_TEMPLATE:
            raise ValueError("Not a fashion templade link")

        visibility = struct.unpack_from(_VISIBILITY_BYTE_FORMAT, b, -2)[0]
        visibility = SkinVisibilityFlag(visibility)

        offset = 1
        skins = {}

        for skin_type in SkinType:
            visible = skin_type.visibility_flag() in visibility
            skin = unpack_skin_from(skin_type, b, offset, visible)
            offset += skin.num_bytes
            skins[skin_type] = skin
        return FashionTemplate(skins)
    
    def __init__(self, skins: dict[SkinType, Skin|DyableSkin]):
        for skin_type in SkinType:
            if skin_type not in skins:
                skins[skin_type] = skin_from_data(skin_type)
        self.skins = skins

    def __eq__(self, other):
        if isinstance(other, FashionTemplate):
            return self.skins == other.skins
        return False

    def diff(self, other: Self):
        return [skin_type
                for skin_type, skin
                in self.skins.items()
                if skin != other.skins[skin_type]]                

    def visibility(self):
        visibility = SkinVisibilityFlag(0)
        for skin in self.skins.values():
            visibility = visibility | skin.visibility_flag()
        return visibility

    def visibility_bytes(self):
        return struct.pack(_VISIBILITY_BYTE_FORMAT, self.visibility().value)
    
    def num_bytes(self):
        n = sum(s.num_bytes for s in self.skins.values())
        n += struct.calcsize(_HEADER_BYTE_FORMAT)
        n += struct.calcsize(_VISIBILITY_BYTE_FORMAT)
        return n
 
    def to_bytes(self):
        buffer = bytearray(self.num_bytes())

        struct.pack_into(_HEADER_BYTE_FORMAT, buffer, 0, ChatLinkType.WARDROBE_TEMPLATE)
        offset = struct.calcsize(_HEADER_BYTE_FORMAT)

        for skin_type in SkinType:
            skin = self.skins[skin_type]
            skin.pack_into(buffer, offset)
            offset += skin.num_bytes

        struct.pack_into(_VISIBILITY_BYTE_FORMAT, buffer, offset, self.visibility().value)

        return buffer
    
    def to_chat_link(self):
        b = self.to_bytes()
        b = base64.b64encode(b)
        link = b.decode('utf-8')
        return f'[&{link}]'

    def __repr__(self):
        return repr(self.skins)
