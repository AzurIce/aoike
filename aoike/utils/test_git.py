import re
import unittest


class TestGit(unittest.TestCase):
    def test_git(self):
        import git
        print(git.get_git_log_commit_list(cwd=r'F:\azurice.github.io'))

    def test_git2(self):
        import git
        print(git.get_git_log_commit_list(cwd=r'D:\_Dev\azurice.github.io', filepath=r'D:\_Dev\azurice.github.io\posts\blog.md'))

if __name__ == '__main__':
    unittest.main()
