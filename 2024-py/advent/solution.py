import logging
from abc import ABC, abstractmethod
from logging import Logger
from typing import Any


class Solution(ABC):
    log: Logger

    def __init__(self) -> None:
        self.log = logging.getLogger(self.__class__.__name__)
        super().__init__()

    @abstractmethod
    def run(self, puzzle_input: str) -> tuple[Any | None, Any | None] | None:
        raise NotImplementedError
