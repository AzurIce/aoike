import os
from pathlib import PurePath

from aoike.utils import files


SRC_DIR = './'
POSTS_DIR = 'posts'
DST_DIR = 'site'

class File:
    filepath: str
    rootpath: str
    """Relative path from SRC_DIR, always separated with '/'"""

    @property
    def basename(self) -> str:
        return os.path.basename(self.filepath)

    @property
    def basename_without_ext(self) -> str:
        return os.path.splitext(self.basename)[0]

    @property
    def url(self) -> str:
        return os.path.normpath(os.path.relpath(self.filepath, self.rootpath))

    @property
    def dst_path(self) -> str:
        return os.path.join(DST_DIR, self.url)

    def __init__(self, filepath: str, rootpath: str = './'):
        self.filepath = PurePath(filepath).as_posix()
        self.rootpath = PurePath(rootpath).as_posix()

    _document = ''
    @property
    def document(self) -> bytes:
        document = ''
        if self._document:
            return self._document
        else:
            with open(os.path.join(self.rootpath, self.filepath), 'rb') as f:
                self._document = f.read()
            return self._document

    def build(self):
        files.write(self.document, self.dst_path)