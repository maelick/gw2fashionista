from enum import IntEnum

class ChatLinkType(IntEnum):
    COIN = 0x01
    ITEM = 0x02
    NPC_TEXT = 0x03
    MAP_LINK = 0x04
    PVP_GAME = 0x05
    SKILL = 0x06
    TRAIT = 0x07
    USER = 0x08
    RECIPE = 0x09
    WARDROBE = 0x0A
    OUTFIT = 0x0B
    WVW_OBJECTIVE = 0x0C
    BUILD_TEMPLATE = 0x0D
    ACHIEVEMENT = 0x0E
    WARDROBE_TEMPLATE = 0x0F
