import os
import unittest


class TestBuild(unittest.TestCase):
    def test_build(self):
        os.chdir(r'F:\azurice.github.io')

        import build
        build.build()
