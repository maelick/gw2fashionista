import logging
import os
import sys
import csv
import json

from gw2fashion.api import GW2API, EquipmentTabFashion

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
        w = csv.writer(f)
        w.writerow(('char_name', 'tab_id', 'tab_name', 'template_link'))
        for t in fashion_templates:
            w.writerow((t.char_name, t.tab_id, t.tab_name, t.fashion_link))

    def write_json_output(self, f, fashion_templates: list[EquipmentTabFashion]):
        json.dump([t.to_dict() for t in fashion_templates], f)
