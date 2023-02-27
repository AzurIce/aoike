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
