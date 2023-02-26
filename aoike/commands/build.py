import os
import time


def build():
    """Perform a full site build."""
    start = time.monotonic()

    for source_dir, dirnames, filenames in os.walk('posts', followlinks=True):
        relative_dir = os.path.relpath(source_dir, 'posts')
        print(f'{source_dir=}, {dirnames=}, {filenames=}')

    print(start)
