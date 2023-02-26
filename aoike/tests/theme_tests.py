import unittest


class Test(unittest.TestCase):
    def test(self):
        import aoike.theme as theme
        theme.theme_dir()
