import os
from pathlib import PurePath

from aoike.commands.build import DST_DIR


class Post:
    """
    A Aoike Post object.
    """
    filepath: str
    """The relative path of the post file from SRC_DIR, always '/' separated"""

    @property
    def basename(self) -> str:
        return os.path.basename(self.filepath)

    @property
    def basename_without_ext(self) -> str:
        return os.path.splitext(self.basename)[0]

    @property
    def url(self) -> str:
        return os.path.normpath(
            os.path.join(os.path.dirname(self.filepath), f'{self.basename_without_ext}.html')
        )

    @property
    def dst_path(self) -> str:
        return os.path.join(DST_DIR, self.url)

    def __init__(self, filepath: str):
        self.filepath = PurePath(filepath).as_posix()

    def content(self) -> str:
        content = ''
        with open(self.filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        return content