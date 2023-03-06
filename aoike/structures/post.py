import os
import re
from io import StringIO
from pathlib import PurePath
from typing import Any, Dict

import jinja2
import markdown
import pymdownx.superfences
from _elementtree import Element
from markdown import Markdown

import aoike.theme
from aoike.structures.file import File
from aoike.utils import files, meta


class Post(File):
    """
    A Aoike Post object.
    """

    def __repr__(self):
        return f'<Post: {self.category=}, {self.url=}>'

    # def __repr__(self):
    #     return super().__repr__() + '\n' \
    #         f'{self.category=}\n' \
    #         f'{self.url=}\n' \
    #         f'{self.meta=}'

    @property
    def category(self):
        return PurePath(os.path.normpath(
            os.path.relpath(os.path.dirname(self.filepath), self.rootpath)
        )).as_posix()

    @property
    def url(self) -> str:
        return PurePath(os.path.normpath(
            os.path.join(
                self.category,
                f'{self.basename_without_ext}.html')
        )).as_posix()

    _document = None
    _meta = None

    @property
    def document(self) -> str:
        if self._document is not None:
            return self._document
        else:
            with open(self.filepath, 'r', encoding='utf-8') as f:
                self._document = f.read()
            return self._document

    @property
    def unmarked_content(self):
        def unmark_element(element: Element, stream=None):
            if stream is None:
                stream = StringIO()
            if element.text:
                stream.write(element.text)
            for sub in element:
                unmark_element(sub, stream)
            if element.tail:
                stream.write(element.tail)
            return stream.getvalue()

        Markdown.output_formats["plain"] = unmark_element
        __md = Markdown(output_format="plain")
        __md.stripTopLevelTags = False

        content = self.content
        content = re.sub(r'\!\[.*?\]\(.*?\)', '\\0', content, 0, re.MULTILINE)
        content = re.sub(r'<img.*?/>', '\\0', content)

        return __md.convert(content)

    @property
    def meta(self) -> Dict[str, Any]:
        if self._document is not None:
            return self._meta
        else:
            self._meta = meta.split_meta(self.document)[0]
            return self._meta

    @property
    def content(self) -> str:
        return meta.split_meta(self.document)[1]

    @property
    def rendered_content(self) -> str:
        return markdown.markdown(self.content, extensions=[
            'pymdownx.arithmatex', 'pymdownx.highlight', 'pymdownx.extra',
            'pymdownx.saneheaders', 'pymdownx.magiclink', 'pymdownx.tasklist',
            'nl2br'
        ], extension_configs={
            'pymdownx.arithmatex': {
                'generic': True,
            },
            'pymdownx.highlight': {
                'linenums': True,
                'use_pygments': True,
                'pygments_style': 'default',
                'auto_title': True,
                'noclasses': True,
                # 'anchor_linenums': True,
                # 'line_spans': '__span',
                'pygments_lang_class': True,
            },
            'pymdownx.extra': {
                'pymdownx.superfences': {
                    'custom_fences': [
                        {
                            'name': 'mermaid',
                            'class': 'mermaid',
                            'format': pymdownx.superfences.fence_div_format
                        }
                    ]
                }
            }
        }, output_format="html")

    def build(self):
        print(f'Building Post: {self.filepath=}, {self.rel_rootpath=}')
        loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
        env = jinja2.Environment(loader=loader, auto_reload=False)
        template = env.get_template('post.html')

        output = template.render(
            {'meta': self.meta, 'content': self.rendered_content, 'rel_rootpath': self.rel_rootpath})

        if output.strip():
            files.write(output.encode('utf-8', errors='xmlcharrefreplace'), self.dst_path)
