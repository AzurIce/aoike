import os
from pathlib import PurePath

from aoike.utils import files


SRC_DIR = './'
POSTS_DIR = 'posts'
DST_DIR = 'site'

class File:
    filepath: str
    """Relative path from SRC_DIR, always separated with '/'"""

    @property
    def basename(self) -> str:
        return os.path.basename(self.filepath)

    @property
    def basename_without_ext(self) -> str:
        return os.path.splitext(self.basename)[0]

    @property
    def url(self) -> str:
        return os.path.normpath(self.filepath)

    @property
    def dst_path(self) -> str:
        return os.path.join(DST_DIR, self.url)

    def __init__(self, filepath: str):
        self.filepath = PurePath(filepath).as_posix()

    def content(self) -> bytes:
        content = ''
        with open(self.filepath, 'rb') as f:
            content = f.read()
        return content

    def build(self):
        content = self.content()
        files.write(content, self.dst_path)