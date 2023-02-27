import os
from pathlib import PurePath

import jinja2

import aoike.theme
from aoike.structures.file import File
from aoike.utils import files


class Post(File):
    """
    A Aoike Post object.
    """

    @property
    def url(self) -> str:
        return os.path.normpath(
            os.path.join(os.path.dirname(self.filepath), f'{self.basename_without_ext}.html')
        )


    def content(self) -> str:
        content = ''
        with open(self.filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        return content

    def build(self):
        loader = jinja2.FileSystemLoader(aoike.theme.get_theme_dir('aoike'))
        env = jinja2.Environment(loader=loader, auto_reload=False)
        template = env.get_template('post.html')

        output = template.render({'content': self.content()})

        if output.strip():
            files.write(output.encode('utf-8', errors='xmlcharrefreplace'), self.dst_path)
