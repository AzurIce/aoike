import re
import unittest


class TestGit(unittest.TestCase):
    def test_git(self):
        import git
        print(git.get_git_log_commit_list(cwd=r'F:\azurice.github.io'))


if __name__ == '__main__':
    unittest.main()
