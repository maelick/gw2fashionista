import unittest

from gw2fashionista import ChatLink
from gw2fashionista.enums.skin import SkinFlag
from gw2fashionista.filter import SkinFilter, ARMORS

import data as testdata

class TestFilterFashion(unittest.TestCase):
    def test_zizi_armor_only(self):
        template = ChatLink.parse(testdata.zizi).fashion_template()
        filter = SkinFilter(ARMORS | SkinFlag.BACKPACK)
        filter.no_underwater()
        template = template.filter(filter)
        self.assertEqual(template.to_chat_link(), testdata.zizi_armor_only)
        expectedTemplate = ChatLink.parse(testdata.zizi_armor_only).fashion_template()
        self.assertEqual(expectedTemplate, template)
