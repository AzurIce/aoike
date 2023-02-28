import os
import unittest


class TestBuild(unittest.TestCase):
    def test_build(self):
        os.chdir(r'D:\_Dev\_Projects\aoike')

        import build
        build.build()