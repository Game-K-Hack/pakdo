import shutil

import pakdo
import pytest
from pathlib import Path

FIXTURES = Path(__file__).resolve().parent / "fixtures"

PNG_MAGIC = b"\x89PNG\r\n\x1a\n"
JPEG_MAGIC = b"\xff\xd8\xff"


def test_convert_jpg_to_png(tmp_path):
    output = tmp_path / "output.png"
    pakdo.convert(str(FIXTURES / "image.jpg"), output=str(output))
    assert output.exists()
    assert output.read_bytes()[:8] == PNG_MAGIC


def test_convert_png_to_jpg(tmp_path):
    output = tmp_path / "output.jpg"
    pakdo.convert(str(FIXTURES / "image.png"), output=str(output))
    assert output.exists()
    assert output.read_bytes()[:3] == JPEG_MAGIC


def test_convert_with_format(tmp_path):
    shutil.copy(FIXTURES / "image.jpg", tmp_path / "image.jpg")
    pakdo.convert(str(tmp_path / "image.jpg"), format="png")
    output = tmp_path / "image.png"
    assert output.exists()
    assert output.read_bytes()[:8] == PNG_MAGIC


def test_convert_no_output_no_format_raises():
    with pytest.raises(pakdo.InvalidArgumentsError, match="output path or a target extension"):
        pakdo.convert(str(FIXTURES / "image.jpg"))


def test_convert_unknown_library_raises():
    with pytest.raises(pakdo.UnknownLibraryError, match="Unknown library"):
        pakdo.convert(
            str(FIXTURES / "image.jpg"),
            output="/tmp/out.png",
            lib="nonexistent",
        )


def test_all_exceptions_inherit_from_base():
    """All pakdo exceptions should be catchable via PakdoBaseError."""
    with pytest.raises(pakdo.PakdoBaseError):
        pakdo.convert(str(FIXTURES / "image.jpg"))
