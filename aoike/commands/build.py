import fnmatch
import os
import time
from typing import Iterable
import jinja2
import aoike.theme
from aoike import utils

SRC_DIR = 'posts'
DST_DIR = 'site'


class Post:
    """
    A Aoike Post object.
    """
    filepath: str

    @property
    def basename(self) -> str:
        return os.path.basename(self.filepath)

    @property
    def basename_without_ext(self) -> str:
        return os.path.splitext(self.basename)[0]

    @property
    def dir_uri(self) -> str:
        return os.path.normpath(os.path.relpath(os.path.dirname(self.filepath), SRC_DIR))

    @property
    def dst_path(self) -> str:
        return os.path.join(DST_DIR, self.dir_uri, f'{self.basename_without_ext}.html')

    def __init__(self, filepath: str):
        self.filepath = filepath

    def content(self) -> str:
        content = ''
        with open(self.filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        return content


def build():
    """Perform a full site build."""
    start = time.monotonic()
    utils.clean_directory(DST_DIR)
    files = []

    for source_dir, dirnames, filenames in os.walk(SRC_DIR, followlinks=True):
        relative_dir = os.path.relpath(source_dir, SRC_DIR)  # Relative path between current dir and SRC_DIR

        # Ignore dirs starts with _
        for dirname in list(dirnames):
            if dirname.startswith('_'):
                dirnames.remove(dirname)
        dirnames.sort()

        for filename in filenames:
            filepath = os.path.normpath(os.path.join(source_dir, filename))
            print(f'{filepath=}')

            # Ignore files starts with _
            if filename.startswith('_'):
                continue

            post = Post(filepath)
            # print(f'{post.filepath=}')
            # print(f'{post.basename=}')
            # print(f'{post.basename_without_ext=}')
            # print(f'{post.dir_uri=}')
            # print(f'{post.dst_path=}\n')

            loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
            env = jinja2.Environment(loader=loader, auto_reload=False)
            template = env.get_template('main.html')

            output = template.render({'content': post.content()})

            if output.strip():
                utils.write_file(output.encode('utf-8', errors='xmlcharrefreplace'), post.dst_path)

        print(f'{source_dir=}, {dirnames=}, {filenames=}')

    print(start)
