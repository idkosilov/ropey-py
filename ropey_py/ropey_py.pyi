"""ropey_py, a text rope implementation for efficient text editing.

This module provides a Python interface to a Rust-based rope data structure
for efficient text manipulation.
"""

from typing import Self

class Rope:
    """A rope data structure for efficient text editing operations.

    A rope is a data structure for efficiently storing and manipulating large
    texts. It provides efficient operations for insertion, deletion, and
    substring extraction.
    """

    def __init__(self, text: str) -> None:
        """Initialize a new Rope with the given text.

        Args:
            text: The initial text content for the rope.
        """

    def len_chars(self) -> int:
        """Get the length of the rope in characters.

        Returns:
            The number of characters in the rope.
        """

    def len_bytes(self) -> int:
        """Get the length of the rope in bytes.

        Returns:
            The number of bytes in the rope.
        """

    def len_lines(self) -> int:
        """Get the number of lines in the rope.

        Returns:
            The number of lines in the rope.
        """

    def char(self, idx: int) -> str:
        """Get the character at the specified index.

        Args:
            idx: The character index.

        Returns:
            The character at the specified index.

        Raises:
            IndexError: If the index is out of range.
        """

    def line(self, line_idx: int) -> str:
        """Get the line at the specified line index.

        Args:
            line_idx: The line index.

        Returns:
            The line at the specified index.

        Raises:
            IndexError: If the line index is out of range.
        """

    def to_string(self) -> str:
        """Convert the rope to a string.

        Returns:
            The string representation of the rope.
        """

    def insert(self, idx: int, text: str) -> None:
        """Insert text at the specified character index.

        Args:
            idx: The character index where to insert the text.
            text: The text to insert.

        Raises:
            IndexError: If the index is out of range.
        """

    def remove(self, start: int, end: int) -> None:
        """Remove text in the specified character range.

        Args:
            start: The start character index (inclusive).
            end: The end character index (exclusive).

        Raises:
            IndexError: If the range is invalid or out of range.
        """

    def split_off(self, at_char: int) -> Self:
        """Split the rope into two at the specified character index.

        Args:
            at_char: The character index where to split.

        Returns:
            A new Rope containing the text after the split point.

        Raises:
            IndexError: If the split index is out of range.
        """

    def slice(self, start: int, end: int) -> str:
        """Get a substring of the rope in the specified character range.

        Args:
            start: The start character index (inclusive).
            end: The end character index (exclusive).

        Returns:
            The substring in the specified range.

        Raises:
            IndexError: If the range is invalid or out of range.
        """

    def byte_slice(self, start_byte: int, end_byte: int) -> str:
        """Get a substring of the rope in the specified byte range.

        Args:
            start_byte: The start byte index (inclusive).
            end_byte: The end byte index (exclusive).

        Returns:
            The substring in the specified byte range.

        Raises:
            IndexError: If the range is invalid or out of range.
        """

    def byte_to_char(self, byte_idx: int) -> int:
        """Convert a byte index to a character index.

        Args:
            byte_idx: The byte index to convert.

        Returns:
            The corresponding character index.

        Raises:
            IndexError: If the byte index is out of range.
        """

    def char_to_byte(self, char_idx: int) -> int:
        """Convert a character index to a byte index.

        Args:
            char_idx: The character index to convert.

        Returns:
            The corresponding byte index.

        Raises:
            IndexError: If the character index is out of range.
        """

    def char_to_line(self, char_idx: int) -> int:
        """Convert a character index to a line index.

        Args:
            char_idx: The character index to convert.

        Returns:
            The line index containing the character.

        Raises:
            IndexError: If the character index is out of range.
        """

    def line_to_char(self, line_idx: int) -> int:
        """Get the character index of the start of a line.

        Args:
            line_idx: The line index.

        Returns:
            The character index of the start of the line.

        Raises:
            IndexError: If the line index is out of range.
        """

    def line_to_byte(self, line_idx: int) -> int:
        """Get the byte index of the start of a line.

        Args:
            line_idx: The line index.

        Returns:
            The byte index of the start of the line.

        Raises:
            IndexError: If the line index is out of range.
        """

    def byte_to_line(self, byte_idx: int) -> int:
        """Convert a byte index to a line index.

        Args:
            byte_idx: The byte index to convert.

        Returns:
            The line index containing the byte.

        Raises:
            IndexError: If the byte index is out of range.
        """
