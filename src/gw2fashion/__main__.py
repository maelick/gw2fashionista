import os
import sys
import csv

from gw2api import GuildWars2Client
from gw2fashion import FashionTemplate, Cache

from dotenv import load_dotenv

import logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(message)s')

def export_fashion(cache: Cache, api):
    chars = api.characters.get()
    logging.info(f'Exporting fashion templates for {len(chars)} characters')
    for c in chars:
        tabs = api.charactersequipmenttabs.get(char_id=c, tabs='all')
        logging.info(f'Exporting {c}\'s {len(tabs)} equipment tabs as fashion templates')
        for i, t in enumerate(tabs):
            equip = t['equipment']
            fill_missing_skins(cache, equip)
            template = FashionTemplate.from_data(equip)
            yield {
                'char_name': c,
                'tab_id': i,
                'tab_name': t['name'],
                'template': template,
                'template_link': template.to_chat_link(),
            }

def fill_missing_skins(cache: Cache, equipment: dict):
    for item in equipment:
        if 'skin' not in item:
            item_data = cache.items.get(item['id'])
            if 'default_skin' in item_data:
                item['skin'] = item_data['default_skin']


if __name__ == '__main__':
    load_dotenv()
    api_key = os.getenv('GW2_API_KEY')
    if not api_key:
        logging.error('GW2 API Key should be provided as an environment variable (GW2_API_KEY)')
        sys.exit(1)

    api = GuildWars2Client(api_key=api_key)
    cache = Cache(api)

    fashion_templates = list(export_fashion(cache, api))

    filename = 'fashion.csv'
    with open(filename, 'w') as f:
        w = csv.writer(f)
        w.writerow(('char_name', 'tab_id', 'tab_name', 'template_link'))
        for t in fashion_templates:
            w.writerow((t['char_name'], t['tab_id'], t['tab_name'], t['template_link']))
    logging.info(f'{len(fashion_templates)} fashion templates written in {filename}')
