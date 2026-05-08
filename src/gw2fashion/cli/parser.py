import argparse

from gw2fashion.cli import commands

def build_parser():
    parser = argparse.ArgumentParser(description='GW2 Fashion Exporter CLI')
    parser.add_argument('-v', '--verbose', help='Increase output verbosity', action='store_true')
    subparser = parser.add_subparsers(dest='command', help='Sub-commands', required=True)
    _export_parser(subparser)
    _read_parser(subparser)
    _merge_parser(subparser)
    _filter_parser(subparser)
    return parser

def _export_parser(subparser):
    parser = subparser.add_parser('export', help='Export equipment tabs as fashion templates using the API.')
    _add_character_list(parser)
    parser.add_argument('--api-key', help='GW2 API key. If missing, it will be read from the environment variable GW2_API_KEY.')
    parser.add_argument('-f', '--format', default='auto', choices=['auto', 'csv', 'json'], help='Output format. If auto, format will be based on the output filename extension and default to CSV if missing filename or unknown extension.')
    parser.add_argument('-o', '--output', help='Filename to use as output.')
    parser.set_defaults(command=commands.Export)
    return parser

def _read_parser(subparser):
    parser = subparser.add_parser('read', help='Read a fashion template and prints its content by retrieving values from the GW2 API.')
    parser.add_argument('chat_links', metavar='fashion-template', nargs='*', help='Chat link of the fashion template(s) to read. If empty, chat links will be read from stdin, either as a CSV file from the column template_link or as one chat_link per row')
    parser.set_defaults(command=commands.Read)
    return parser

def _merge_parser(subparser):
    parser = subparser.add_parser('merge', help='Merge two fashion templates by overriding specific parts of the first one with values of the second one')
    parser.add_argument('base_fashion_template', metavar='fashion-template1', help='Chat link of the base fashion template to override.')
    parser.add_argument('new_fashion_template', metavar='fashion-template2', help='Chat link of the fashion template with new values to apply to the base one.')
    _add_filters(parser)
    parser.set_defaults(command=commands.Merge)
    return parser

def _filter_parser(subparser):
    parser = subparser.add_parser('filter', help='Filter a fashion template to include only specific parts.')
    _add_filters(parser)
    parser.set_defaults(command=commands.Filter)
    return parser

def _add_character_list(parser: argparse.ArgumentParser, mandatory=False):
    help_text = 'List of character names.'
    if not mandatory:
        help_text += ' (If empty, uses all characters.)'
    parser.add_argument('characters', nargs='+' if mandatory else '*', help=help_text)

def _add_filters(parser: argparse.ArgumentParser):
    group = parser.add_argument_group('Filtering', 'Determines which parts of the fashion template to filter.')
    group.add_argument('--no-weapons', action='store_true')
    group.add_argument('--no-armor', action='store_true')
    group.add_argument('--no-backpack', action='store_true')
    group.add_argument('--no-oufit', action='store_true')

    subgroup = group.add_mutually_exclusive_group()
    subgroup.add_argument('--no-underwater', action='store_true')
    subgroup.add_argument('--only-underwater', action='store_true')
