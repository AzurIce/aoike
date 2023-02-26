import os.path
from importlib.metadata import entry_points


def get_theme_dir(name: str) -> str:
    theme = get_themes()[name]
    return os.path.dirname(os.path.abspath(theme.load().__file__))


def get_themes():
    themes = {}
    eps = entry_points(group='aoike.themes')
    builtins = {ep.name for ep in eps if ep.dist is not None and ep.dist.name == 'aoike'}

    for theme in eps:
        themes[theme.name] = theme

    return themes
