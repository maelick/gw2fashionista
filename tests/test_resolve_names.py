import unittest

from gw2fashionista import ChatLink, Skin, DyableSkin
from gw2fashionista.template import FashionTemplateData
from gw2fashionista.api import GW2API

import data as testdata

class TestResolveNames(unittest.TestCase):
    api = GW2API()

    def test_empty(self):
        fashion = ChatLink.parse(testdata.empty).fashion_template().to_data()
        self.api.cache_fashion_data([fashion])
        data = self.api.resolve_fashion_data(fashion)

        for skin_type, skin in data.__dict__.items():
            self.assertIsNone(skin, f'{skin_type} should be None')

    def test_zizi(self):
        fashion = ChatLink.parse(testdata.zizi).fashion_template().to_data()
        self.api.cache_fashion_data([fashion])
        data = self.api.resolve_fashion_data(fashion)

        self.assertIsNotNone(data.outfit) # TODO

        self.assertEqual(data.aquabreather.name, 'Black Earth Aquabreather')

        self.assertEqual(data.weapon_aquatic_a.name, 'Steam Speargun')
        self.assertEqual(data.weapon_aquatic_b.name, 'Iron Spear')
        self.assertEqual(data.weapon_a1.name, 'Quip')
        self.assertEqual(data.weapon_a2.name, 'Quip')
        self.assertEqual(data.weapon_b1.name, 'The Dreamer')
        self.assertIsNone(data.weapon_b2)

        self.assertZiziArmor(data)

    def test_zizi_armor_only(self):
        fashion = ChatLink.parse(testdata.zizi_armor_only).fashion_template().to_data()
        self.api.cache_fashion_data([fashion])
        data = self.api.resolve_fashion_data(fashion)

        self.assertIsNone(data.aquabreather)
        self.assertIsNone(data.outfit)
        self.assertIsNone(data.weapon_aquatic_a)
        self.assertIsNone(data.weapon_aquatic_b)
        self.assertIsNone(data.weapon_a1)
        self.assertIsNone(data.weapon_a2)
        self.assertIsNone(data.weapon_b1)
        self.assertIsNone(data.weapon_b2)

        self.assertZiziArmor(data)

    def assertZiziArmor(self, data: FashionTemplateData):
        self.assertEqual(data.backpack.name, 'Pink Quaggan Backpack')
        self.assertEqual(data.backpack.dye1.name, 'Dye Remover')
        self.assertEqual(data.backpack.dye2.name, 'Dye Remover')
        self.assertEqual(data.backpack.dye3.name, 'Dye Remover')
        self.assertEqual(data.backpack.dye4.name, 'Dye Remover')

        self.assertEqual(data.chest.name, 'Sneakthief Coat')
        self.assertEqual(data.chest.dye1.name, 'Electro Pink')
        self.assertEqual(data.chest.dye2.name, 'Permafrost')
        self.assertEqual(data.chest.dye3.name, 'Permafrost')
        self.assertEqual(data.chest.dye4.name, 'Dye Remover')

        self.assertEqual(data.shoes.name, 'Sneakthief Sandals')
        self.assertEqual(data.shoes.dye1.name, 'Electro Pink')
        self.assertEqual(data.shoes.dye2.name, 'Permafrost')
        self.assertEqual(data.shoes.dye3.name, 'Dye Remover')
        self.assertEqual(data.shoes.dye4.name, 'Dye Remover')

        self.assertEqual(data.gloves.name, 'Noble Gloves')
        self.assertEqual(data.gloves.dye1.name, 'Dye Remover')
        self.assertEqual(data.gloves.dye2.name, 'Permafrost')
        self.assertEqual(data.gloves.dye3.name, 'Electro Pink')
        self.assertEqual(data.gloves.dye4.name, 'Electro Pink')

        self.assertEqual(data.head.name, 'Fuzzy Cat Hat')
        self.assertEqual(data.head.dye1.name, 'Electro Pink')
        self.assertEqual(data.head.dye2.name, 'Permafrost')
        self.assertEqual(data.head.dye3.name, 'Dye Remover')
        self.assertEqual(data.head.dye4.name, 'Dye Remover')

        self.assertEqual(data.legs.name, 'Sneakthief Leggings')
        self.assertEqual(data.legs.dye1.name, 'Dye Remover')
        self.assertEqual(data.legs.dye2.name, 'Permafrost')
        self.assertEqual(data.legs.dye3.name, 'Permafrost')
        self.assertEqual(data.legs.dye4.name, 'Dye Remover')

        self.assertEqual(data.shoulders.name, 'Shoulder Scarf')
        self.assertEqual(data.shoulders.dye1.name, 'Electro Pink')
        self.assertEqual(data.shoulders.dye2.name, 'Permafrost')
        self.assertEqual(data.shoulders.dye3.name, 'Dye Remover')
        self.assertEqual(data.shoulders.dye4.name, 'Dye Remover')
