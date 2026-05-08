import unittest

from gw2fashion import ChatLink, Skin, DyableSkin
from gw2fashion.enums.skin import SkinType

import data as testdata

class TestParseTemplate(unittest.TestCase):
    def test_empty_link(self):
        with self.assertRaisesRegex(ValueError, r'empty base64 content'):
            ChatLink.parse('')
        with self.assertRaisesRegex(ValueError, r'empty base64 content'):
            ChatLink.parse('[&]')

    def test_not_chat_link(self):
        with self.assertRaisesRegex(ValueError, r'not a GW2 chat link'):
            ChatLink.parse('This is not a chat link')
        with self.assertRaisesRegex(ValueError, r'not a GW2 chat link'):
            ChatLink.parse('[&This is not a chat link]')

    def test_invalid_base64(self):
        with self.assertRaisesRegex(ValueError, r'Invalid base64'):
            ChatLink.parse('hello')
        with self.assertRaisesRegex(ValueError, r'Invalid base64'):
            ChatLink.parse('[&hello]')

    def test_invalid_link_type(self):
        with self.assertRaisesRegex(ValueError, r'not a valid ChatLinkType'):
            ChatLink.parse('abcd')
        with self.assertRaisesRegex(ValueError, r'not a valid ChatLinkType'):
            ChatLink.parse('[&abcd]')

    def test_invalid_length(self):
        link = 'DwAAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAEAAQABAAEAAAABAAEAAQABAAAAAQABAAEAAQAAAAAAAAAAAAD/fw=='
        with self.assertRaisesRegex(ValueError, r'should be 97 bytes long'):
            ChatLink.parse(link)
        with self.assertRaisesRegex(ValueError, r'should be 97 bytes long'):
            ChatLink.parse('[&' + link + ']')

    def test_empty(self):
        template = ChatLink.parse(testdata.empty).fashion_template()
        self.assertEqual(len(template.skins), len(SkinType))
        for skin in template.skins.values():
            self.assertSkinNotSet(skin)
            self.assertSkinVisible(skin)

        template_base64only = ChatLink.parse(testdata.empty[2:-1]).fashion_template()
        self.assertEqual(template, template_base64only)
        
        self.assertEqual(testdata.empty, template.to_chat_link())

    def test_zizi(self):
        template = ChatLink.parse(testdata.zizi).fashion_template()
        self.assertEqual(len(template.skins), len(SkinType))
        for skin_type, skin in template.skins.items():
            match skin_type:
                case SkinType.WEAPON_B2:
                    self.assertSkinNotSet(skin)
                    self.assertSkinVisible(skin)
                case SkinType.OUTFIT:
                    self.assertSkinSet(skin) # TODO double check if correct
                    self.assertSkinVisible(skin)
                    self.assertHasNoDyes(skin)
                case SkinType.AQUABREATHER:
                    self.assertSkinSet(skin)
                    self.assertSkinNotVisible(skin)
                case SkinType.BACKPACK:
                    self.assertSkinSet(skin)
                    self.assertSkinVisible(skin)
                    self.assertHasNoDyes(skin)
                case SkinType.CHEST | SkinType.SHOES | SkinType.LEGS | SkinType.GLOVES | SkinType.HEAD | SkinType.SHOULDERS:
                    self.assertSkinSet(skin)
                    self.assertSkinVisible(skin)
                    self.assertHasDyes(skin)
                case _:
                    self.assertSkinSet(skin)
                    self.assertSkinVisible(skin)
        
        # self.assertEqual(testdata.zizi, template.to_chat_link()) # TODO FIXME

    def test_zizi_armor_only(self):
        template = ChatLink.parse(testdata.zizi_armor_only).fashion_template()
        self.assertEqual(len(template.skins), len(SkinType))
        for skin_type, skin in template.skins.items():
            match skin_type:
                case SkinType.OUTFIT:
                    self.assertSkinNotSet(skin)
                    self.assertSkinVisible(skin)
                    self.assertHasNoDyes(skin)
                case SkinType.BACKPACK:
                    self.assertSkinSet(skin)
                    self.assertSkinVisible(skin)
                    self.assertHasNoDyes(skin)
                case SkinType.CHEST | SkinType.SHOES | SkinType.LEGS | SkinType.GLOVES | SkinType.HEAD | SkinType.SHOULDERS:
                    self.assertSkinSet(skin)
                    self.assertSkinVisible(skin)
                    self.assertHasDyes(skin)
                case _:
                    self.assertSkinNotSet(skin)
                    self.assertSkinVisible(skin)
        
        self.assertEqual(testdata.zizi_armor_only, template.to_chat_link())

    def assertSkinSet(self, skin: Skin):
        self.assertNotEqual(skin.skin, 0, msg=f'{skin.skin_type} should be set')

    def assertSkinNotSet(self, skin: Skin):
        self.assertEqual(skin.skin, 0, msg=f'{skin.skin_type} should not be set')

    def assertSkinVisible(self, skin: Skin):
        self.assertTrue(skin.visible, msg=f'{skin.skin_type} should be visible')

    def assertSkinNotVisible(self, skin: Skin):
        self.assertFalse(skin.visible, msg=f'{skin.skin_type} should not be visible')
    
    def assertHasDyes(self, skin: DyableSkin):
        self.assertNotEqual(skin.dyes, (1, 1, 1, 1), msg=f'{skin.skin_type} should have dyes')

    def assertHasNoDyes(self, skin: DyableSkin):
        self.assertEqual(skin.dyes, (1, 1, 1, 1), msg=f'{skin.skin_type} should not have dyes')
