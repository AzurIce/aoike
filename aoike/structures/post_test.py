import unittest


class PostTest(unittest.TestCase):
    def test_properties(self):
        from post import Post
        post = Post(r'F:\Dev\aoike\posts\Notes\使用哈希算法以及加盐来增强服务端密码存储安全.md', r'F:\Dev\aoike\posts')
        print(f'{post}\n')

        post = Post(r'F:\Dev\aoike\posts\Notes\Golang\Golang 接口.md', r'F:\Dev\aoike\posts')
        print(f'{post}')


if __name__ == '__main__':
    unittest.main()
