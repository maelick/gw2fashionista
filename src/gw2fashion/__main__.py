from dotenv import load_dotenv
from gw2fashion.cli import CLI

if __name__ == '__main__':
    load_dotenv()
    _ = CLI().run()
