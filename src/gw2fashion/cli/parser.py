import argparse

from gw2fashion.cli import commands

def build_parser():
    parser = argparse.ArgumentParser(description='GW2 Fashion Exporter CLI')
    parser.add_argument('-v', '--verbose', help='Increase output verbosity', action='store_true')
    subparser = parser.add_subparsers(dest='command', help='Sub-commands', required=True)
    _export_parser(subparser)
    return parser

def _export_parser(subparser):
    parser = subparser.add_parser('export', help='Export equipment tabs as fashion templates using the API.')
    _add_character_list(parser)
    parser.add_argument('--api-key', help='GW2 API key. If missing, it will be read from the environment variable GW2_API_KEY.')
    parser.add_argument('-f', '--format', default='auto', choices=['auto', 'csv', 'json'], help='Output format. If auto, format will be based on the output filename extension and default to CSV if missing filename or unknown extension.')
    parser.add_argument('-o', '--output', help='Filename to use as output.')
    parser.set_defaults(command=commands.Export)
    return parser

def _add_character_list(parser: argparse.ArgumentParser, mandatory=False):
    help_text = 'List of character names.'
    if not mandatory:
        help_text += ' (If empty, uses all characters.)'
    parser.add_argument('characters', nargs='+' if mandatory else '*', help=help_text)
