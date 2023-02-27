import fnmatch
import os
import posixpath
import time
from pathlib import PurePath
from typing import Iterable
import jinja2
import aoike.theme
import aoike.utils.files
from urllib.parse import quote

from aoike.structures.file import File
from aoike.structures.post import Post

SRC_DIR = './'
POSTS_DIR = 'posts'
DST_DIR = 'site'


def build():
    """Perform a full site build."""
    start = time.monotonic()
    aoike.utils.files.clean_directory(DST_DIR)

    files = _get_files()
    for file in files:
        print(f'{type(file)}')
        file.build()

    loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
    env = jinja2.Environment(loader=loader, auto_reload=False)
    template = env.get_template('main.html')

    output = template.render({'posts': [file for file in files if isinstance(file, Post)]})

    if output.strip():
        aoike.utils.files.write(output.encode('utf-8', errors='xmlcharrefreplace'), os.path.join(DST_DIR, 'index.html'))

    print('Built in %.2f seconds', time.monotonic() - start)


def _get_files() -> Iterable[File]:
    files = []

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

            if filename.endswith('.md'):
                print(f'{filename=}')
                files.append(Post(filepath))
            else:
                print(f'{filename=}')
                files.append(File(filepath))

            # print(f'{post.filepath=}')
            # print(f'{post.basename=}')
            # print(f'{post.basename_without_ext=}')
            # print(f'{post.dir_uri=}')
            # print(f'{post.dst_path=}\n')
    return files
