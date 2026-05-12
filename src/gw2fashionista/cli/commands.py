import logging
import os
import sys
import csv
import json

from gw2fashionista.api import GW2API, EquipmentTabFashion
from gw2fashionista.chatlink import ChatLink
from gw2fashionista.template import FashionTemplate
from gw2fashionista.filter import SkinFilter

class BaseCommand:
    def __init__(self, args):
        self.args = args

    def __call__(self):
        print(self.args)
        raise NotImplementedError
    
    def get_api(self):
        api_key = self.args.api_key or os.getenv('GW2_API_KEY')
        if not api_key:
            logging.error('GW2 API key needs to be provided as an environment variable (GW2_API_KEY) or CLI argument')
            sys.exit(1)

        api = GW2API(api_key=api_key)
        return api


class FilterBaseCommand(BaseCommand):
    def __init__(self, args):
        super().__init__(args)
        self.init_filter()

    def init_filter(self):
        self.filter = SkinFilter()
        if self.args.no_weapons:
            self.filter.no_weapons()
        if self.args.no_armor:
            self.filter.no_armor()
        if self.args.no_backpack:
            self.filter.no_backpack()
        if self.args.no_outfit:
            self.filter.no_outfit()
        if self.args.no_underwater:
            self.filter.no_underwater()
        if self.args.only_underwater:
            self.filter.only_underwater()


class Export(FilterBaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.export_equipment_fashion()

    def export_equipment_fashion(self):
        api = self.get_api()

        fashion_templates = [t.extract_fashion(self.filter) for t in api.fetch_equipment_tabs(self.args.characters)]
        if self.args.add_default_names:
            for t in fashion_templates:
                if not t.tab_name:
                    t.tab_name = f'{t.char_name} {t.tab_id}'

        with self.open_dest() as f:
            self.write_output(f, fashion_templates)

    def open_dest(self):
        if self.args.output:
            logging.info(f'Writing output to {self.args.output}')
            return open(self.args.output, 'w')
        return sys.stdout

    def output_format(self):
        if self.args.format != 'auto':
            return self.args.format
        if self.args.output:
            match os.path.splitext(self.args.output)[1]:
                case '.csv':
                    return 'csv'
                case '.json':
                    return 'json'
        return 'csv'

    def write_output(self, f, fashion_templates: list[EquipmentTabFashion]):
        output_format = self.output_format()
        match output_format:
            case 'csv':
                self.write_csv_output(f, fashion_templates)
            case 'json':
                self.write_json_output(f, fashion_templates)
            case _:
                # Should never happen => if it does, self.output_format is wrong
                raise ValueError('Invalid output type: ' + output_format)

    def write_csv_output(self, f, fashion_templates: list[EquipmentTabFashion]):
        w = csv.DictWriter(f, fieldnames=['char_name', 'tab_id', 'tab_name', 'fashion_link'])
        w.writeheader()
        for t in fashion_templates:
            w.writerow(t.to_dict())

    def write_json_output(self, f, fashion_templates: list[EquipmentTabFashion]):
        json.dump([t.to_dict() for t in fashion_templates], f)


class Read(BaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.read_templates()

    def read_templates(self):
        api = GW2API()
        fashion = {link: self.read_template(link).to_data() for link in self.get_chat_links()}
        api.cache_fashion_data(fashion.values())
        for f in fashion.values():
            api.resolve_fashion_data(f)

        result = [{'chat_link': link, 'skins': data.to_dict()} for link, data in fashion.items()]
        json.dump(result, sys.stdout)

    def read_template(self, chat_link: str) -> FashionTemplate:
        try:
            parsed_chat_link = ChatLink.parse(chat_link)
        except Exception as e:
            logging.error(f'Invalid fashion template: {e}')
            sys.exit(1) # TODO maybe we should be able to configure whether to exit or continue on error
        try:
            return parsed_chat_link.fashion_template()
        except TypeError:
            logging.error(f'Chat link is not a fashion template: {parsed_chat_link.type}')
            sys.exit(1) # TODO maybe we should be able to configure whether to exit or continue on error

    def get_chat_links(self):
        if self.args.chat_links:
            return self.args.chat_links
        return self.read_chat_links(sys.stdin)

    def read_chat_links(self, f):
        rows = [row for row in csv.reader(f)]
        if not rows:
            return []
        if len(rows[0]) == 1:
            return get_column(rows)
        try:
            col = rows[0].index('fashion_link')
        except ValueError as e:
            raise ValueError('Missing column fashion_link in input CSV file') from e
        return get_column(rows[1:], col)

class Merge(FilterBaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.merge_templates()

    def merge_templates(self):
        base_fashion = parse_fashion(self.args.base_fashion_template)
        new_fashion = parse_fashion(self.args.new_fashion_template).filter(self.filter)
        merged = base_fashion.merge(new_fashion, self.args.ignore_skin, self.args.ignore_dyes)
        print(merged.to_chat_link())


class Filter(FilterBaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.filter_template()

    def filter_template(self):
        fashion = parse_fashion(self.args.fashion_template)
        filtered = fashion.filter(self.filter)
        print(filtered.to_chat_link())


def get_column(rows, col=0):
    return [link[col] for link in rows if len(link)]

def parse_fashion(chat_link: str):
    return ChatLink.parse(chat_link).fashion_template()
