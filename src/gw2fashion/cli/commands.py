import logging
import os
import sys
import csv
import json

from gw2fashion.api import GW2API, EquipmentTabFashion
from gw2fashion.enums.chatlink import ChatLinkType
from gw2fashion.chatlink import ChatLink
from gw2fashion.template import FashionTemplate

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


class Export(BaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.export_equipment_fashion()

    def export_equipment_fashion(self):
        api = self.get_api()

        fashion_templates = [t.extract_fashion() for t in api.fetch_equipment_tabs(self.args.characters)]

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
        templates = [self.read_template(chat_link) for chat_link in self.get_chat_links()]
        data = [api.resolve_fashion_data(t).to_dict() for t in templates]
        json.dump(data, sys.stdout)

    def read_template(self, chat_link: str) -> FashionTemplate:
        try:
            parsed_chat_link = ChatLink.parse(chat_link)
        except Exception as e:
            logging.error(f'Invalid fashion template: {e}')
            sys.exit(1) # TODO maybe we should be able to configure whether to exit or continue on error
        if parsed_chat_link.type != ChatLinkType.WARDROBE_TEMPLATE:
            logging.error(f'Chat link is not a fashion template: {parsed_chat_link.type}')
            sys.exit(1) # TODO maybe we should be able to configure whether to exit or continue on error
        return parsed_chat_link.object

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


def get_column(rows, col=0):
    return [link[col] for link in rows if len(link)]

class Merge(BaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.merge_templates()

    def merge_templates(self):
        raise NotImplementedError()


class Filter(BaseCommand):
    def __init__(self, args):
        super().__init__(args)

    def __call__(self):
        self.filter_template()

    def filter_template(self):
        raise NotImplementedError()
