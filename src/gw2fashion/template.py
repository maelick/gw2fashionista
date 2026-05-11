import struct
import base64
from typing import Self, Optional
from dataclasses import dataclass

from gw2fashion.enums.skin import SkinType, SkinFlag
from gw2fashion.filter import SkinFilter
from gw2fashion.skins import SkinData, DyableSkinData, Skin, DyableSkin, skin_from_data, unpack_skin_from, skin_type_from_equipment_slot
from gw2fashion.enums.chatlink import ChatLinkType

_HEADER_BYTE_FORMAT = '<B'
_VISIBILITY_BYTE_FORMAT = '<H'

@dataclass
class FashionTemplateData:
    aquabreather: Optional[SkinData]
    backpack: Optional[DyableSkinData]
    chest: Optional[DyableSkinData]
    shoes: Optional[DyableSkinData]
    gloves: Optional[DyableSkinData]
    head: Optional[DyableSkinData]
    legs: Optional[DyableSkinData]
    shoulders: Optional[DyableSkinData]
    outfit: Optional[DyableSkinData]
    weapon_aquatic_a: Optional[SkinData]
    weapon_aquatic_b: Optional[SkinData]
    weapon_a1: Optional[SkinData]
    weapon_a2: Optional[SkinData]
    weapon_b1: Optional[SkinData]
    weapon_b2: Optional[SkinData]

    def all_skins(self):
        return (skin for skin in (
            self.aquabreather,
            self.backpack,
            self.chest,
            self.shoes,
            self.gloves,
            self.head,
            self.legs,
            self.shoulders,
            self.weapon_aquatic_a,
            self.weapon_aquatic_b,
            self.weapon_a1,
            self.weapon_a2,
            self.weapon_b1,
            self.weapon_b2,
        ) if skin is not None)

    def all_dyable_skins(self):
        return (skin for skin in (
            self.backpack,
            self.chest,
            self.shoes,
            self.gloves,
            self.head,
            self.legs,
            self.shoulders,
            self.outfit,
        ) if skin is not None)

    def all_skin_ids(self):
        return set(skin.id for skin in self.all_skins())

    def all_color_ids(self):
        return set(color.id for skin in self.all_dyable_skins() for color in skin.all_dyes())

    def to_dict(self):
        return {k: v.to_dict() for k, v in self.__dict__.items() if v is not None}


class FashionTemplate:
    @classmethod
    def from_data(cls, equipment_tab: list[dict]):
        # TODO outfit missing from API data
        skins = {}
        for item in equipment_tab:
            skin_type = skin_type_from_equipment_slot(item['slot'])
            if skin_type:
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
        visibility = SkinFlag(visibility)

        offset = 1
        skins = {}

        for skin_type in SkinType:
            visible = skin_type.flag() in visibility
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

    def filter(self, filter: SkinFilter):
        for skin_type in filter.hidden_skins():
            self.skins[skin_type] = skin_from_data(skin_type)

    def apply(self, other: Self):
        for skin_type, skin in other.skins.items():
            self.skins[skin_type] = skin

    def visibility(self):
        visibility = SkinFlag(0)
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

    def to_data(self):
        return FashionTemplateData(
            self.skins[SkinType.AQUABREATHER].to_data(),
            self.skins[SkinType.BACKPACK].to_data(),
            self.skins[SkinType.CHEST].to_data(),
            self.skins[SkinType.SHOES].to_data(),
            self.skins[SkinType.GLOVES].to_data(),
            self.skins[SkinType.HEAD].to_data(),
            self.skins[SkinType.LEGS].to_data(),
            self.skins[SkinType.SHOULDERS].to_data(),
            self.skins[SkinType.OUTFIT].to_data(),
            self.skins[SkinType.WEAPON_AQUATIC_A].to_data(),
            self.skins[SkinType.WEAPON_AQUATIC_B].to_data(),
            self.skins[SkinType.WEAPON_A1].to_data(),
            self.skins[SkinType.WEAPON_A2].to_data(),
            self.skins[SkinType.WEAPON_B1].to_data(),
            self.skins[SkinType.WEAPON_B2].to_data(),
        )

    def __repr__(self):
        return repr(self.skins)
