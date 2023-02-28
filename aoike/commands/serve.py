import logging
import tempfile

log = logging.getLogger(__name__)


def serve():
    """
    start the Aoike development server
    """
    site_dir = tempfile.mkdtemp(prefix='aoike_')
