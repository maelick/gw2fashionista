import re
import base64
from dataclasses import dataclass
from typing import Any

from gw2fashionista.template import FashionTemplate
from gw2fashionista.enums.chatlink import ChatLinkType

BASE64_RE = r'[-A-Za-z0-9+/]*={0,3}'
CHAT_LINK_RE = re.compile(r'^\[?&?(' + BASE64_RE + r')\]?$')


@dataclass
class ChatLink:
    type: ChatLinkType
    object: Any

    @classmethod
    def parse(cls, link: str):
        b = decode_chat_link(link)
        link_type = ChatLinkType(b[0])
        match link_type:
            case ChatLinkType.WARDROBE_TEMPLATE:
                return cls(link_type, FashionTemplate.from_bytes(b))
            case _:
                raise ValueError(f'Unsupported chat link header: {b[0]}')

    def fashion_template(self) -> FashionTemplate:
        if not isinstance(self.object, FashionTemplate):
            raise TypeError('Not a FashionTemplate')
        return self.object


def decode_chat_link(link: str):
    m = CHAT_LINK_RE.match(link)
    if not m or len(m.groups()) == 0:
        raise ValueError(f'{repr(link)} is not a GW2 chat link')
    b = m.groups()[0].encode('utf-8')
    try:
        b = base64.b64decode(b)
    except Exception as e:
        raise ValueError(f'{repr(link)}: {e}') from e
    if len(b) == 0:
        raise ValueError(f'{repr(link)} has empty base64 content')
    return b
