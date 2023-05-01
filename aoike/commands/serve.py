import logging
import tempfile

log = logging.getLogger(__name__)


def serve(*, src_dir):
    """
    start the Aoike development server
    """
    site_dir = tempfile.mkdtemp(prefix='aoike_')
