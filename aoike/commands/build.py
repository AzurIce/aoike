from datetime import datetime, time
import fnmatch
import os
import time
from pathlib import PurePath
from pprint import pprint
from typing import Iterable, List

import jinja2

import aoike.theme
import aoike.utils.files
import aoike.utils.git as git
from aoike.structures.file import File
from aoike.structures.post import Post

POSTS_DIR = 'posts'
DST_DIR = 'site'


def _get_post_key(post: Post):
    # print(post.meta)
    create, update, title = post.meta['create'], post.meta['title']
    return create, update, title


def build(*, src_dir):
    """Perform a full site build."""
    start = time.monotonic()
    aoike.utils.files.clean_directory(DST_DIR)

    files = _get_files(src_dir=src_dir)
    posts: List[Post] = [file for file in files if isinstance(file, Post)]
    git_tracked_posts = [post for post in posts if len(post.git_log_commits)]
    print(git_tracked_posts)
    # print(f'Before sort: {git_tracked_posts}')
    list.sort(git_tracked_posts, key=_get_post_key, reverse=True)
    # print(f'After sort: {git_tracked_posts}')
    files = [file for file in files if file not in posts]

    for post in git_tracked_posts:
        # print(f'{type(post)}, {post.url=}, {post.filepath=}, {post.rootpath=}')
        post.build()

    for file in files:
        # print(f'{type(file)}, {file.url=}, {file.filepath=}, {file.rootpath=}')
        file.build()

    categories = {}
    for post in git_tracked_posts:
        if post.category not in categories:
            categories[post.category] = [_post for _post in git_tracked_posts if _post.category == post.category]

    categories = dict(sorted(categories.items(), key=lambda x: x[0]))
    pprint(categories)

    tags = {}
    for post in git_tracked_posts:
        if 'tags' in post.meta:
            post_tags = post.meta['tags']
            for post_tag in post_tags:
                if post_tag not in tags:
                    tags[post_tag] = [_post for _post in git_tracked_posts if 'tags' in _post.meta and post_tag in _post.meta['tags']]
    pprint(tags)

    loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
    env = jinja2.Environment(loader=loader, auto_reload=False)

    template = env.get_template('main.html')
    output = template.render({
        'posts': git_tracked_posts,
        'categories': categories,
        'rel_rootpath': '.',
        'commits': git.get_git_log_commit_list()
    })

    if output.strip():
        aoike.utils.files.write(output.encode('utf-8', errors='xmlcharrefreplace'), os.path.join(DST_DIR, 'index.html'))

    template = env.get_template('categories.html')
    output = template.render({'posts': git_tracked_posts, 'categories': categories, 'rel_rootpath': '.', 'tags': tags})
    if output.strip():
        aoike.utils.files.write(
            output.encode('utf-8', errors='xmlcharrefreplace'), os.path.join(DST_DIR, 'categories.html')
        )

    print(f'Built in {time.monotonic() - start} seconds')


def _get_files(*, src_dir) -> list[File]:
    files = []

    for source_dir, dirnames, filenames in os.walk(os.path.join(src_dir, POSTS_DIR), followlinks=True):

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
                files.append(Post(filepath, src_dir))
            else:
                print(f'{filename=}')
                files.append(File(filepath, src_dir))

    theme_dir = aoike.theme.get_theme_dir('aoike')
    loader = jinja2.FileSystemLoader(theme_dir)
    env = jinja2.Environment(loader=loader, auto_reload=False)


    def filter(name):
        patterns = ['.*', '*/.*', '*.py', '*.pyc', '*.html', '*readme*']
        for pattern in patterns:
            if fnmatch.fnmatch(name.lower(), pattern):
                return False
        return True

    for path in env.list_templates(filter_func=filter):
        # Theme files do not override docs_dir files
        path = PurePath(path).as_posix()
        # print(f'{path=}')
        if path not in [file.url for file in files]:
            if os.path.isfile(os.path.join(theme_dir, path)):
                files.append(File(os.path.join(theme_dir, path), theme_dir))

        # print(f'{post.filepath=}')
        # print(f'{post.basename=}')
        # print(f'{post.basename_without_ext=}')
        # print(f'{post.dir_uri=}')
        # print(f'{post.dst_path=}\n')

    return files
