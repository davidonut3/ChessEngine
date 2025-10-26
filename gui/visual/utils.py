from config import *

class Color:
    RED = (255, 0, 0)
    GREEN = (0, 255, 0)
    BLUE = (0, 0, 255)
    YELLOW = (245, 212, 66)

    BLACK = (0, 0, 0)
    WHITE = (255, 255, 255)
    BG = (60, 60, 60)

    DARK = (168, 100, 50)
    LIGHT = (189, 134, 87)

    DGREEN = (105, 181, 119)
    LGREEN = (145, 217, 158)
    LTAKE = (100, 100, 100)
    DTAKE = (80, 80, 80)

    ACTIVE = (34, 156, 67)

TILE_SIZE = 60
BOARD_SIZE = TILE_SIZE * 8
BOARD_X, BOARD_Y = 60, 60
SCREEN_WIDTH, SCREEN_HEIGHT = BOARD_SIZE + BOARD_X * 2, BOARD_SIZE + BOARD_Y * 2

piece_to_letter = {
    WHITE: {
        'ROOK': 'R',
        'KNIGHT': 'N',
        'BISHOP': 'B',
        'QUEEN': 'Q',
        'KING': 'K',
        'PAWN': 'P',
    },
    BLACK: {
        'ROOK': 'r',
        'KNIGHT': 'n',
        'BISHOP': 'b',
        'QUEEN': 'q',
        'KING': 'k',
        'PAWN': 'p',
    }
}

letter_to_name = {
    'R': 'ROOK',
    'N': 'KNIGHT',
    'B': 'BISHOP',
    'Q': 'QUEEN',
    'K': 'KING',
    'P': 'PAWN',
    'r': 'ROOK',
    'n': 'KNIGHT',
    'b': 'BISHOP',
    'q': 'QUEEN',
    'k': 'KING',
    'p': 'PAWN',
    '-': NO_PIECE,
}

letter_to_type = {
    'R': WHITE,
    'N': WHITE,
    'B': WHITE,
    'Q': WHITE,
    'K': WHITE,
    'P': WHITE,
    'r': BLACK,
    'n': BLACK,
    'b': BLACK,
    'q': BLACK,
    'k': BLACK,
    'p': BLACK,
}

letter_to_bounds = {
    'R': [12, 10, 9, 8],
    'N': [8, 8, 7, 7],
    'B': [7, 6, 7, 7],
    'Q': [4, 6, 4, 5],
    'K': [7, 6, 6, 6],
    'P': [15, 9, 12, 7],
    'r': [12, 10, 9, 8],
    'n': [8, 8, 7, 7],
    'b': [7, 6, 7, 7],
    'q': [4, 6, 4, 5],
    'k': [7, 6, 6, 6],
    'p': [15, 9, 12, 7],
}

rank_to_string = {
    0: '8',
    1: '7',
    2: '6',
    3: '5',
    4: '4',
    5: '3',
    6: '2',
    7: '1',
}

file_to_string = {
    0: 'a',
    1: 'b',
    2: 'c',
    3: 'd',
    4: 'e',
    5: 'f',
    6: 'g',
    7: 'h',
}

string_to_rank = {
    '1': 7,
    '2': 6,
    '3': 5,
    '4': 4,
    '5': 3,
    '6': 2,
    '7': 1,
    '8': 0,
}

string_to_file = {
    'a': 0,
    'b': 1,
    'c': 2,
    'd': 3,
    'e': 4,
    'f': 5,
    'g': 6,
    'h': 7,
}

def in_bounds(rankfile, bounds=[0, 7, 0, 7]):
    return rankfile[0] >= bounds[0] and rankfile[1] >= bounds[2] and rankfile[0] <= bounds[1] and rankfile[1] <= bounds[3]

def set_in_bounds(rankfile, bounds=[0, 7, 0, 7]):
    return (min(max(rankfile[0], bounds[0]), bounds[1]), min(max(rankfile[1], bounds[2]), bounds[3]))

def get_board_pos(rankfile):
    return (floor(rankfile[0] / (TILE_SIZE)), floor(rankfile[1] / (TILE_SIZE)))

def get_screen_pos(rankfile):
    return (rankfile[0] * TILE_SIZE, rankfile[1] * TILE_SIZE)

def invert_pos(pos):
    return (BOARD_SIZE - pos[0], BOARD_SIZE - pos[1])

def invert_tile(pos):
    return (BOARD_SIZE - pos[0] - TILE_SIZE, BOARD_SIZE - pos[1] - TILE_SIZE)

def distance(pos1, pos2):
    return sqrt((pos2[0] - pos1[0]) ** 2 + (pos2[1] - pos1[1]) ** 2)

def convert_mouse(pos, turn):
    pos = (pos[0] - BOARD_X, pos[1] - BOARD_Y)
    return pos if turn == WHITE else invert_pos(pos)

def tile_to_string(rankfile):
    return file_to_string[rankfile[1]] + rank_to_string[rankfile[0]]

def move_to_lan(rankfile1, rankfile2, promotion=''):
    return tile_to_string(rankfile1) + tile_to_string(rankfile2) + promotion

def lan_to_move(lan):
    return (string_to_rank[lan[1]], string_to_file[lan[0]]), (string_to_rank[lan[3]], string_to_file[lan[2]])

def load_index_text(font: pygame.font):
    ranks = [
        font.render('8', True, Color.BLACK),
        font.render('7', True, Color.BLACK),
        font.render('6', True, Color.BLACK),
        font.render('5', True, Color.BLACK),
        font.render('4', True, Color.BLACK),
        font.render('3', True, Color.BLACK),
        font.render('2', True, Color.BLACK),
        font.render('1', True, Color.BLACK),
    ]

    files = [
        font.render('a', True, Color.BLACK),
        font.render('b', True, Color.BLACK),
        font.render('c', True, Color.BLACK),
        font.render('d', True, Color.BLACK),
        font.render('e', True, Color.BLACK),
        font.render('f', True, Color.BLACK),
        font.render('g', True, Color.BLACK),
        font.render('h', True, Color.BLACK),
    ]

    return ranks, files