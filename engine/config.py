"""
Configuration module for the chess engine frontend.

- Handles imports
- Defines constants
"""

import os

# we disable the welcome text from the pygame library
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"
import pygame

import time
import random
import multiprocessing
from math import floor, sqrt
from os import getcwd
from rust_utils import FenPy, BotV1Py, move_gen_perft_py, perft_check

MAIN_LOCATION = getcwd()

WHITE = 'w'
BLACK = 'b'
NO_PIECE = 0

# BATCH_SIZE * 2 * time per player: the number of minutes running matches will take at least
GAMES = 1020
BATCH_SIZE = 17

WIN = {
    '1-0': 'white wins',
    '0-1': 'black wins',
    '½-½': 'draw',
}

USER = 'user'

DEFAULT = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"