import re
import subprocess
from datetime import datetime
from typing import List

re_git_commit = r'commit (.*)\nAuthor: (.*) <(.*)>\nDate: *(.*)\n\n *(.*)\n'


class GitLogCommitInfo:

    def __repr__(self):
        return f'{self.id=}, {self.username=}, {self.email=}, {self.date=}, {self.comment=}'

    def __init__(self, id: str, username: str, email: str, date: str, comment: str):
        self.id = id
        self.username = username
        self.email = email
        self.date = datetime.strptime(date, '%a %b %d %H:%M:%S %Y %z')
        self.comment = comment


def get_git_log_commit_list(*, cwd=None) -> List[GitLogCommitInfo]:
    res = subprocess.run(['git', 'log'], capture_output=True, encoding='utf-8', check=True)
    res = re.findall(re_git_commit, res.stdout)
    return [GitLogCommitInfo(*commit_info) for commit_info in res]
