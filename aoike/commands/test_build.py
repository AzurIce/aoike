import os
import unittest


class TestBuild(unittest.TestCase):
    def test_build(self):
        os.chdir(r'../../')

        import build
        build.build()
