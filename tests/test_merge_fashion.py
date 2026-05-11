import unittest

from gw2fashion import ChatLink
from gw2fashion.enums.skin import SkinType

import data as testdata

class TestMergeFashion(unittest.TestCase):
    def test_zizi_armor_on_peekaboo(self):
        peekaboo = ChatLink.parse(testdata.peekaboo).fashion_template()
        zizi_armor = ChatLink.parse(testdata.zizi_armor_only).fashion_template()
        merged = peekaboo.merge(zizi_armor)
        for skin_type, skin in merged.skins.items():
            match skin_type:
                case SkinType.CHEST | SkinType.LEGS | SkinType.GLOVES | SkinType.HEAD | SkinType.SHOULDERS | SkinType.SHOES | SkinType.BACKPACK:
                    self.assertEqual(zizi_armor.skins[skin_type], skin, f'{skin_type} should be the same as new')
                case _:
                    self.assertEqual(peekaboo.skins[skin_type], skin, f'{skin_type} should be the same as original')
