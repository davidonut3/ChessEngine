import pygame
from math import floor, sqrt

BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
DARK = (168, 100, 50)
LIGHT = (189, 134, 87)
YELLOW = (245, 212, 66)
BG = (60, 60, 60)
DGREEN = (105, 181, 119)
LGREEN = (145, 217, 158)
LTAKE = (100, 100, 100)
DTAKE = (80, 80, 80)

NO_PIECE = 0

pygame.init()

tile_size = 60
board_size = tile_size * 8
board_x, board_y = 60, 60
screen_width, screen_height = board_size + board_x * 2, board_size + board_y * 2
board_screen = pygame.surface.Surface((board_size, board_size))
screen = pygame.display.set_mode((screen_width, screen_height))
pygame.display.set_caption('Chess')
pygame.display.set_icon(pygame.image.load('chess.png'))

def in_bounds(pos):
    return pos[0] >= 0 and pos[1] >= 0 and pos[0] < 8 and pos[1] < 8

def get_board_pos(pos):
    return (floor(pos[0] / (tile_size)), floor(pos[1] / (tile_size)))

def get_screen_pos(pos):
    return (pos[0] * tile_size, pos[1] * tile_size)

def invert_pos(pos):
    return (board_size - pos[0], board_size - pos[1])

def invert_tile(pos):
    return (board_size - pos[0] - tile_size, board_size - pos[1] - tile_size)

def distance(pos1, pos2):
    return sqrt((pos2[0] - pos1[0]) ** 2 + (pos2[1] - pos1[1]) ** 2)

def convert_mouse(pos, turn):
    pos = (pos[0] - board_x, pos[1] - board_y)
    return pos if turn == 'W' else invert_pos(pos)


class Piece:
    def __init__(self, name, type, bounds, pos=(0,0)):
        self.name = name
        self.type = type
        self.bounds = bounds
        self.pos = pos
        self.image = pygame.image.load(f"pieces/{type}_{name}.png").convert_alpha()
        self.image = pygame.transform.scale_by(self.image, tile_size / 60)
        self.image_width = self.image.get_width()
        self.image_height = self.image.get_height()
    
    def draw(self, screen, turn):
        draw_pos = self.pos if turn == 'W' else invert_tile(self.pos)
        screen.blit(self.image, (draw_pos[0], draw_pos[1]))

    def center_image_at(self, pos, turn):
        self.pos = (pos[0] - self.image_width / 2, pos[1] - self.image_height / 2)
        if turn == 'W':
            self.pos = (max(min(self.pos[0], board_size - self.image_height + self.bounds[2]), -self.bounds[0]), max(min(self.pos[1], board_size - self.image_height + self.bounds[3]), -self.bounds[1]))
        else:
            self.pos = (max(min(self.pos[0], board_size - self.image_height + self.bounds[0]), -self.bounds[2]), max(min(self.pos[1], board_size - self.image_height + self.bounds[1]), -self.bounds[3]))

class Tile:
    def __init__(self, pos, has_piece, piece, color):
        self.pos = pos
        self.draw_pos = get_screen_pos(self.pos)
        self.has_piece = has_piece
        self.piece = piece
        if self.has_piece:
            self.piece.pos = self.draw_pos
        self.color = color
        self.default_color = color
        self.border = False
        self.circle = False
        self.active = False
        self.to_be_moved = False
        self.to_be_moved_to = False

    def draw_border(self, screen, pos):
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1], tile_size, 3))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1], 3, tile_size))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0] + tile_size - 3, pos[1], 3, tile_size))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1] + tile_size - 3, tile_size, 3))
    
    def draw_take(self, screen, pos):
        first = tile_size * 1/4
        second = tile_size * 3/4

        points = [(pos[0], pos[1]), (pos[0] + first - 1, pos[1]), (pos[0], pos[1] + first - 1)]
        pygame.draw.polygon(screen, DTAKE, points)
        points = [(pos[0] + 2, pos[1] + 2), (pos[0] + first - 1 - 6, pos[1] + 2), (pos[0] + 2, pos[1] + first - 1 - 6)]
        pygame.draw.polygon(screen, LTAKE, points)

        points = [(pos[0] + second - 1, pos[1]), (pos[0] + tile_size - 1, pos[1]), (pos[0] + tile_size - 1, pos[1] + first - 1)]
        pygame.draw.polygon(screen, DTAKE, points)
        points = [(pos[0] + second - 1 + 6, pos[1] + 2), (pos[0] + tile_size - 1 - 2, pos[1] + 2), (pos[0] + tile_size - 1 - 2, pos[1] + first - 1 - 6)]
        pygame.draw.polygon(screen, LTAKE, points)

        points = [(pos[0], pos[1] + second - 1), (pos[0], pos[1] + tile_size - 1), (pos[0] + first - 1, pos[1] + tile_size - 1)]
        pygame.draw.polygon(screen, DTAKE, points)
        points = [(pos[0] + 2, pos[1] + second - 1 + 6), (pos[0] + 2, pos[1] + tile_size - 1 - 2), (pos[0] + first - 1 - 6, pos[1] + tile_size - 1 - 2)]
        pygame.draw.polygon(screen, LTAKE, points)

        points = [(pos[0] + second - 1, pos[1] + tile_size - 1), (pos[0] + tile_size - 1, pos[1] + tile_size - 1), (pos[0] + tile_size - 1, pos[1] + second - 1)]
        pygame.draw.polygon(screen, DTAKE, points)
        points = [(pos[0] + second - 1 + 6, pos[1] + tile_size - 1 - 2), (pos[0] + tile_size - 1 - 2, pos[1] + tile_size - 1 - 2), (pos[0] + tile_size - 1 - 2, pos[1] + second - 1 + 6)]
        pygame.draw.polygon(screen, LTAKE, points)

    def draw(self, screen, turn):
        draw_pos = self.draw_pos if turn == 'W' else invert_tile(self.draw_pos)
        if self.active:
            pygame.draw.rect(screen, YELLOW, pygame.Rect(draw_pos[0], draw_pos[1], tile_size, tile_size))
        else:
            pygame.draw.rect(screen, self.default_color, pygame.Rect(draw_pos[0], draw_pos[1], tile_size, tile_size))
        if self.circle and not self.has_piece:
            pygame.draw.circle(screen, LTAKE, (draw_pos[0] + tile_size / 2, draw_pos[1] + tile_size / 2), tile_size / 5)
            pygame.draw.circle(screen, DTAKE, (draw_pos[0] + tile_size / 2, draw_pos[1] + tile_size / 2), tile_size / 5, 3)
        elif self.circle:
            self.draw_take(screen, draw_pos)
        if self.border:
            self.draw_border(screen, draw_pos)

    def set_piece(self, piece):
        self.has_piece = True
        self.piece = piece
        self.piece.pos = self.draw_pos

    def remove_piece(self):
        self.has_piece = False
        self.piece = NO_PIECE

    def set_display(self, color, border, circle):
        self.color = color
        self.border = border
        self.circle = circle

    def set_active(self):
        self.active = True

    def reset(self):
        self.border = False
        self.circle = False
        self.active = False
        self.color = self.default_color


board_setup = [[Tile((i,j), False, NO_PIECE, LIGHT if (i % 2 and j % 2) or (not i % 2 and not j % 2) else DARK) for i in range(8)] for j in range(8)]

bounds = {
    'ROOK': [12, 10, 9, 8],
    'KNIGHT': [8, 8, 7, 7],
    'BISHOP': [7, 6, 7, 7],
    'QUEEN': [4, 6, 4, 5],
    'KING': [7, 6, 6, 6],
    'PAWN': [15, 9, 12, 7]
    }

board_setup[0][0].set_piece(Piece('ROOK', 'B', bounds['ROOK']))
board_setup[0][1].set_piece(Piece('KNIGHT', 'B', bounds['KNIGHT']))
board_setup[0][2].set_piece(Piece('BISHOP', 'B', bounds['BISHOP']))
board_setup[0][3].set_piece(Piece('QUEEN', 'B', bounds['QUEEN']))
board_setup[0][4].set_piece(Piece('KING', 'B', bounds['KING']))
board_setup[0][5].set_piece(Piece('BISHOP', 'B', bounds['BISHOP']))
board_setup[0][6].set_piece(Piece('KNIGHT', 'B', bounds['KNIGHT']))
board_setup[0][7].set_piece(Piece('ROOK', 'B', bounds['ROOK']))

board_setup[7][0].set_piece(Piece('ROOK', 'W', bounds['ROOK']))
board_setup[7][1].set_piece(Piece('KNIGHT', 'W', bounds['KNIGHT']))
board_setup[7][2].set_piece(Piece('BISHOP', 'W', bounds['BISHOP']))
board_setup[7][3].set_piece(Piece('QUEEN', 'W', bounds['QUEEN']))
board_setup[7][4].set_piece(Piece('KING', 'W', bounds['KING']))
board_setup[7][5].set_piece(Piece('BISHOP', 'W', bounds['BISHOP']))
board_setup[7][6].set_piece(Piece('KNIGHT', 'W', bounds['KNIGHT']))
board_setup[7][7].set_piece(Piece('ROOK', 'W', bounds['ROOK']))

for i in range(8):
    board_setup[1][i].set_piece(Piece('PAWN', 'B', bounds['PAWN']))
    board_setup[6][i].set_piece(Piece('PAWN', 'W', bounds['PAWN']))


class GameState:
    def __init__(self):
        self.board = board_setup
        self.current_tile = board_setup[0][0]
        self.current_tile_active = False
        self.turn = 'W'
        self.screen_mouse = (0,0)
        self.board_mouse = (0,0)
        self.in_promotion = False
        self.previous_move = [0, 0]
        self.show_previous_move = False

    def is_legit_move(self, dest: Tile):
        tile = self.current_tile
        piece = tile.piece
        if dest.has_piece:
            if piece.type == dest.piece.type:
                return False
                
        if tile.piece.name == 'PAWN' and tile.piece.type == 'W':
            if dest.has_piece:
                if (dest.pos[0] == tile.pos[0] - 1 or dest.pos[0] == tile.pos[0] + 1) and dest.pos[1] == tile.pos[1] - 1:
                    return True
            else:
                if dest.pos[0] == tile.pos[0] and dest.pos[1] == tile.pos[1] - 1:
                    return True
                elif dest.pos[0] == tile.pos[0] and dest.pos[1] == tile.pos[1] - 2 and not self.board[tile.pos[1] - 1][tile.pos[0]].has_piece and tile.pos[1] == 6:
                    return True
            return False
        
        if tile.piece.name == 'PAWN' and tile.piece.type == 'B':
            if dest.has_piece:
                if (dest.pos[0] == tile.pos[0] - 1 or dest.pos[0] == tile.pos[0] + 1) and dest.pos[1] == tile.pos[1] + 1:
                    return True
            else:
                if dest.pos[0] == tile.pos[0] and dest.pos[1] == tile.pos[1] + 1:
                    return True
                elif dest.pos[0] == tile.pos[0] and dest.pos[1] == tile.pos[1] + 2 and not self.board[tile.pos[1] + 1][tile.pos[0]].has_piece and tile.pos[1] == 1:
                    return True
            return False

        if tile.piece.name == 'ROOK' or tile.piece.name == 'QUEEN':
            if dest.pos[0] == tile.pos[0]:
                distance = dest.pos[1] - tile.pos[1]
                direction = -1 if distance < 0 else 1
                for i in range(1, abs(distance)):
                    tile = self.board[tile.pos[1] + i * direction][tile.pos[0]]
                    if tile.to_be_moved_to or (tile.has_piece and not tile.to_be_moved):
                        return False
                return True
            elif dest.pos[1] == tile.pos[1]:
                distance = dest.pos[0] - tile.pos[0]
                direction = -1 if distance < 0 else 1
                for i in range(1, abs(distance)):
                    tile = self.board[tile.pos[1]][tile.pos[0] + i * direction]
                    if tile.to_be_moved_to or (tile.has_piece and not tile.to_be_moved):
                        return False
                return True
            else:
                if tile.piece.name == 'ROOK':
                    return False
            
        if tile.piece.name == 'KNIGHT':
            if ((dest.pos[0] == tile.pos[0] + 1 or dest.pos[0] == tile.pos[0] - 1) and (dest.pos[1] == tile.pos[1] + 2 or dest.pos[1] == tile.pos[1] - 2)) or ((dest.pos[0] == tile.pos[0] + 2 or dest.pos[0] == tile.pos[0] - 2) and (dest.pos[1] == tile.pos[1] + 1 or dest.pos[1] == tile.pos[1] - 1)):
                return True
            return False

        if tile.piece.name == 'BISHOP' or tile.piece.name == 'QUEEN':
            if dest.pos[0] - tile.pos[0] == dest.pos[1] - tile.pos[1]:
                distance = dest.pos[0] - tile.pos[0]
                direction = -1 if distance < 0 else 1
                for i in range(1, abs(distance)):
                    tile = self.board[tile.pos[1] + i * direction][tile.pos[0] + i * direction]
                    if tile.to_be_moved_to or (tile.has_piece and not tile.to_be_moved):
                        return False
                return True
            elif dest.pos[0] - tile.pos[0] == -dest.pos[1] + tile.pos[1]:
                distance = dest.pos[0] - tile.pos[0]
                direction = -1 if distance < 0 else 1
                for i in range(1, abs(distance)):
                    tile = self.board[tile.pos[1] + i * -direction][tile.pos[0] + i * direction]
                    if tile.to_be_moved_to or (tile.has_piece and not tile.to_be_moved):
                        return False
                return True
            else:
                return False

        if tile.piece.name == 'KING':
            if (dest.pos[0] == tile.pos[0] - 1 or dest.pos[0] == tile.pos[0] or dest.pos[0] == tile.pos[0] + 1) and (dest.pos[1] == tile.pos[1] - 1 or dest.pos[1] == tile.pos[1] or dest.pos[1] == tile.pos[1] + 1):
                return True
            return False

    def check_check(self, start, dest):
        king = self.get_current_king()
        opponents = self.get_opponent_tiles()
        self.turn = 'W' if self.turn == 'B' else 'B'
        start.to_be_moved = True
        dest.to_be_moved_to = True
        for tile in opponents:
            if self.is_legit_move(king):
                return True
        return False

    def get_current_king(self):
        for row in self.board:
            for tile in row:
                if tile.has_piece:
                    if tile.piece.name == 'KING' and tile.piece.type == self.turn:
                        return tile
        raise(Exception('???'))

    def get_opponent_tiles(self):
        tiles = []
        for row in self.board:
            for tile in row:
                if tile.has_piece:
                    if tile.piece.type != self.turn:
                        tiles.append(tile)
        return tiles
    
    def check_promotion(self):
        tile = self.current_tile
        piece = tile.piece
        piece = self.current_tile.piece
        if (piece.name == 'PAWN' and piece.type == 'W' and tile.pos[1] == 0) or (piece.name == 'PAWN' and piece.type == 'B' and tile.pos[1] == 7):
            self.in_promotion = True

    def select_piece(self):
        if in_bounds(self.board_mouse):
            selected_tile: Tile = self.board[self.board_mouse[1]][self.board_mouse[0]]
            if selected_tile.has_piece and selected_tile.piece.type == self.turn:
                self.current_tile = selected_tile
                self.current_tile_active = True
                self.show_possible_moves()
                self.current_tile.set_active()

    def place_piece(self):
        piece = self.current_tile.piece
        dest = self.board[self.board_mouse[1]][self.board_mouse[0]]
        legit_move = game.is_legit_move(dest)
        self.reset_colors()
        if legit_move:
            self.current_tile.to_be_moved = True
            dest.to_be_moved_to = True
            self.turn = 'W' if self.turn == 'B' else 'B'
            if not self.check_check(self.current_tile, dest):
                self.current_tile.to_be_moved = False
                dest.to_be_moved_to = False
                if self.show_previous_move:
                    self.previous_move[0].reset()
                    self.previous_move[1].reset()
                else:
                    self.show_previous_move = True
                self.current_tile.remove_piece()
                self.previous_move[0] = self.current_tile
                self.current_tile = self.board[self.board_mouse[1]][self.board_mouse[0]]
                self.previous_move[1] = self.current_tile
                self.turn = 'W' if self.turn == 'B' else 'B'
        self.current_tile.set_piece(piece)
        self.current_tile_active = False
        if legit_move:
            self.check_promotion()

    def draw_board(self):
        if self.show_previous_move:
            start = self.previous_move[0]
            end = self.previous_move[1]
            start.set_display(LGREEN, True, start.circle)
            end.set_display(DGREEN, True, end.circle)
        for row in self.board:
            for tile in row:
                tile.draw(board_screen, self.turn)
        for row in self.board:
            for tile in row:
                if tile.has_piece:
                    tile.piece.draw(board_screen, self.turn)
        self.current_tile.piece.draw(board_screen, self.turn)

    def get_possible_moves(self):
        possible_moves = []
        for row in self.board:
            for tile in row:
                if self.is_legit_move(tile):
                    possible_moves.append(tile)
        return possible_moves

    def show_possible_moves(self):
        for tile in self.get_possible_moves():
            tile.set_display(tile.color, tile.border, True)
    
    def reset_colors(self, full=False):
        for row in self.board:
            for tile in row:
                if not tile in self.previous_move or full:
                    tile.reset()


game = GameState()


pygame.mouse.set_pos(board_size / 2, board_size / 2)
clock = pygame.time.Clock()
running = True

while running:
    game.screen_mouse = convert_mouse(pygame.mouse.get_pos(), game.turn)
    game.board_mouse = get_board_pos(game.screen_mouse)

    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        if event.type == pygame.MOUSEBUTTONDOWN:
            if not game.current_tile_active:
                game.select_piece()
        if event.type == pygame.MOUSEBUTTONUP:
            if game.current_tile_active:
                game.place_piece()

    if game.current_tile_active:
        game.current_tile.piece.center_image_at(game.screen_mouse, game.turn)

    screen.fill(BG)
    board_screen.fill(WHITE)

    game.draw_board()

    screen.blit(board_screen, (board_x, board_y))

    pygame.display.update()
    clock.tick(60)

pygame.quit()