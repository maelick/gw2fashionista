import logging
from gw2fashion.cli.parser import build_parser

class CLI:
    def __init__(self):
        self.parser = build_parser()

    def run(self, args=None):
        args = self.parser.parse_args(args)
        log_level = logging.INFO if args.verbose else logging.WARNING
        logging.basicConfig(level=log_level, format='%(asctime)s - %(message)s')
        cmd = args.command(args)
        return cmd()
