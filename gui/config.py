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
from engine import FenPy, move_gen_perft_py, perft_check, moves_per_second_perft_py

MAIN_LOCATION = getcwd()

WHITE = 'w'
BLACK = 'b'
NO_PIECE = 0

# BATCH_SIZE * 2 * time per player: the number of minutes running matches will take at least
GAMES = 100
BATCH_SIZE = 10

WHITE_WINS = 'WhiteWins'
BLACK_WINS = 'BlackWins'
DRAW = 'Draw'

USER = 'user'

DEFAULT = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"