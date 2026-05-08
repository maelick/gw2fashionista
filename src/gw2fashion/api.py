import logging
from dataclasses import dataclass
from typing import Generator, Any, Optional

from gw2api import GuildWars2Client
from gw2fashion import FashionTemplate, Cache


@dataclass
class BaseEquipmentTab:
    char_name: str
    tab_id: int
    tab_name: str

    def to_dict(self):
        return {k: v for k, v in self.__dict__.items() if not k.startswith('_')}

@dataclass
class EquipmentTab(BaseEquipmentTab):
    equipment: dict

    def fill_missing_skins(self, cache: Cache):
        for item in self.equipment:
            if 'skin' not in item:
                item_data = cache.items.get(item['id'])
                if 'default_skin' in item_data:
                    item['skin'] = item_data['default_skin']

    def extract_fashion(self):
        template = FashionTemplate.from_data(self.equipment)
        return EquipmentTabFashion(self.char_name, self.tab_id, self.tab_name, template, template.to_chat_link())


@dataclass
class EquipmentTabFashion(BaseEquipmentTab):
    _fashion_template: FashionTemplate
    fashion_link: str


class GW2API:
    def __init__(self, api_key: Optional[str]=None):
        self.client = GuildWars2Client(api_key=api_key)
        self.cache = Cache(self.client)
        if api_key:
            self.account_name = self.client.account.get()['name']
            logging.info(f'Logged in towards GW2 API with account {self.account_name}')
        else:
            self.account_name = None

    def fetch_equipment_tabs(self, characters: list[str]) -> Generator[EquipmentTab, Any, None]:
        if not characters:
            logging.info('Retrieving character list from GW2 API')
            characters = self.client.characters.get()
        logging.info(f'Retrieving equipment tabs for {len(characters)} characters')
        
        for c in characters:
            for t in self.fetch_char_equipment_tabs(c):
                yield t
    
    def fetch_char_equipment_tabs(self, char_name: str):
        tabs = self.client.charactersequipmenttabs.get(char_id=char_name, tabs='all')
        logging.info(f'Retrieved {char_name}\'s {len(tabs)} equipment tabs')
        for i, t in enumerate(tabs):
            tab = EquipmentTab(char_name, i, t['name'], t['equipment'])
            tab.fill_missing_skins(self.cache)
            yield tab
