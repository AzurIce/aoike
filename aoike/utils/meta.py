import re
from typing import Dict, Any, Tuple

import yaml
from yaml import SafeLoader

FRONT_MATTER_RE = re.compile(r'^---[ \t]*\n((?:.*[ \t]*\n)*?)---[ \t]*\n')


def split_meta(doc: str) -> Tuple[Dict[str, Any], str]:
    front_matter = FRONT_MATTER_RE.match(doc)
    if front_matter:
        meta = yaml.load(front_matter.group(1), SafeLoader)
        return meta, doc[front_matter.end():]
    return {}, doc