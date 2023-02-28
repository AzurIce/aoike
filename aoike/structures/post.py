import os
from pathlib import PurePath

import jinja2
import markdown

import aoike.theme
from aoike.structures.file import File
from aoike.utils import files, meta

from typing import Tuple, Any, Dict


class Post(File):
    """
    A Aoike Post object.
    """

    @property
    def url(self) -> str:
        return os.path.normpath(
            os.path.join(os.path.dirname(self.filepath), f'{self.basename_without_ext}.html')
        )

    _document = ''
    @property
    def document(self) -> str:
        if self._document:
            return self._document
        else:
            with open(self.filepath, 'r', encoding='utf-8') as f:
                self._document = f.read()
            return self._document


    @property
    def meta(self) -> Dict[str, Any]:
        return meta.split_meta(self.document)[0]

    @property
    def content(self) -> str:
        return meta.split_meta(self.document)[1]

    @property
    def rendered_content(self) -> str:
        return markdown.markdown(self.content, extensions=[
            'pymdownx.arithmatex', 'pymdownx.highlight', 'pymdownx.extra'
        ], extension_configs={
            'pymdownx.arithmatex': {
                'generic': True,
            },
            'pymdownx.highlight': {
                'linenums': True,
                'use_pygments': False,
            },
        })

    def build(self):
        loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
        env = jinja2.Environment(loader=loader, auto_reload=False)
        template = env.get_template('post.html')

        output = template.render({'meta': self.meta, 'content': self.rendered_content})

        if output.strip():
            files.write(output.encode('utf-8', errors='xmlcharrefreplace'), self.dst_path)
