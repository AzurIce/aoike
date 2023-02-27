import fnmatch
import os
import posixpath
import time
from pathlib import PurePath
from typing import Iterable
import jinja2
import aoike.theme
from aoike import utils
from urllib.parse import quote

SRC_DIR = './'
POSTS_DIR = 'posts'
DST_DIR = 'site'


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


def build():
    """Perform a full site build."""
    start = time.monotonic()
    utils.clean_directory(DST_DIR)

    posts = _get_posts()
    for post in posts:
        print(f'{post.filepath=}')
        print(f'{post.url=}')
        _build_post(post)

    loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
    env = jinja2.Environment(loader=loader, auto_reload=False)
    template = env.get_template('main.html')

    output = template.render({'posts': posts})

    if output.strip():
        utils.write_file(output.encode('utf-8', errors='xmlcharrefreplace'), os.path.join(DST_DIR, 'index.html'))

    print('Built in %.2f seconds', time.monotonic() - start)


def _build_post(post: Post):
    loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
    env = jinja2.Environment(loader=loader, auto_reload=False)
    template = env.get_template('post.html')

    output = template.render({'content': post.content()})

    if output.strip():
        utils.write_file(output.encode('utf-8', errors='xmlcharrefreplace'), post.dst_path)


def _get_posts() -> Iterable[Post]:
    posts = []

    for source_dir, dirnames, filenames in os.walk(POSTS_DIR, followlinks=True):

        # Ignore dirs starts with _
        for dirname in list(dirnames):
            if dirname.startswith('_'):
                dirnames.remove(dirname)
        dirnames.sort()

        for filename in filenames:
            filepath = os.path.normpath(os.path.join(source_dir, filename))
            # print(f'{filepath=}')

            # Ignore files starts with _
            if filename.startswith('_'):
                continue

            posts.append(Post(filepath))
            # print(f'{post.filepath=}')
            # print(f'{post.basename=}')
            # print(f'{post.basename_without_ext=}')
            # print(f'{post.dir_uri=}')
            # print(f'{post.dst_path=}\n')
    return posts
