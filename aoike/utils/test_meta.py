import unittest


class MetaTest(unittest.TestCase):
    def test_get_meta(self):
        import files
        import meta
        doc = files.read_str(r'F:\Dev\aoike\posts\提高生产力的工具们（持续更新）.md')
        print(meta.get_meta(doc))
