import os
import unittest


class TestBuild(unittest.TestCase):
    def test_build(self):
        # os.chdir(r'D:\_Dev\aoike')
        os.chdir(r'H:\Projects\aoike')

        import build
        # build.build(src_dir=r'D:\_Dev\azurice.github.io')
        build.build(src_dir=r'F:\azurice.github.io')
