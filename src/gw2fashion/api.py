import logging
from dataclasses import dataclass
from typing import Generator, Any, Optional
from collections.abc import Iterable

from gw2api import GuildWars2Client
from gw2fashion.cache import Cache
from gw2fashion.template import FashionTemplate, FashionTemplateData
from gw2fashion.skins import SkinData, DyableSkinData, ColorData


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

    def cache_fashion_data(self, templates: Iterable[FashionTemplateData]):
        missing_skins = set()
        missing_colors = set()
        missing_outfits = set()
        for t in templates:
            missing_skins.update(t.all_skin_ids())
            missing_colors.update(t.all_color_ids())
            if t.outfit is not None:
                missing_outfits.add(t.outfit.id)

        self.cache.skins.fetch_missings(list(missing_skins))
        self.cache.colors.fetch_missings(list(missing_colors))
        self.cache.outfits.fetch_missings(list(missing_outfits))

    def resolve_fashion_data(self, data: FashionTemplateData):
        self.resolve_skin_names(data)
        self.resolve_skin_dye_names(data)
        return data

    def resolve_skin_names(self, data: FashionTemplateData):
        self.resolve_skin_name(data.aquabreather)
        self.resolve_skin_name(data.backpack)
        self.resolve_skin_name(data.chest)
        self.resolve_skin_name(data.shoes)
        self.resolve_skin_name(data.gloves)
        self.resolve_skin_name(data.head)
        self.resolve_skin_name(data.legs)
        self.resolve_skin_name(data.shoulders)
        self.resolve_skin_name(data.weapon_aquatic_a)
        self.resolve_skin_name(data.weapon_aquatic_b)
        self.resolve_skin_name(data.weapon_a1)
        self.resolve_skin_name(data.weapon_a2)
        self.resolve_skin_name(data.weapon_b1)
        self.resolve_skin_name(data.weapon_b2)
        self.resolve_outfit_name(data.outfit)

    def resolve_skin_dye_names(self, data: FashionTemplateData):
        self.resolve_dye_names(data.backpack)
        self.resolve_dye_names(data.chest)
        self.resolve_dye_names(data.shoes)
        self.resolve_dye_names(data.gloves)
        self.resolve_dye_names(data.head)
        self.resolve_dye_names(data.legs)
        self.resolve_dye_names(data.shoulders)
        self.resolve_dye_names(data.outfit)

    def resolve_skin_name(self, skin: Optional[SkinData]):
        if skin is not None:
            skin.name = self.cache.skins.get(skin.id)['name']

    def resolve_outfit_name(self, outfit: Optional[SkinData]):
        if outfit is not None:
            outfit.name = self.cache.outfits.get(outfit.id)['name']

    def resolve_dye_names(self, skin: Optional[DyableSkinData]):
        if skin is not None:
            self.resolve_dye_name(skin.dye1)
            self.resolve_dye_name(skin.dye2)
            self.resolve_dye_name(skin.dye3)
            self.resolve_dye_name(skin.dye4)

    def resolve_dye_name(self, dye: ColorData):
        dye.name = self.cache.colors.get(dye.id)['name']
