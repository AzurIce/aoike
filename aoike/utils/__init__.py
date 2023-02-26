import os
import shutil


def write_file(content: bytes, filepath: str):
    """
    Write content to filepath, making sure any parent directories exist.
    """
    output_dir = os.path.dirname(filepath)
    os.makedirs(output_dir, exist_ok=True)
    with open(filepath, 'wb') as f:
        f.write(content)


def clean_directory(directory: str) -> None:
    """
    Remove the content of a directory recursively but not the directory itself.
    """
    if not os.path.exists(directory):
        return

    for entry in os.listdir(directory):
        # Don't remove hidden files from the directory. We never copy files
        # that are hidden, so we shouldn't delete them either.
        if entry.startswith('.'):
            continue

        path = os.path.join(directory, entry)
        if os.path.isdir(path):
            shutil.rmtree(path, True)
        else:
            os.unlink(path)
