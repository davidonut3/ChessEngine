from config import *
from visual.utils import *
from visual.piece import Piece

class Tile:
    def __init__(self, pos=(0,0), size=60, has_piece=False, piece: Piece=NO_PIECE, color=Color.LIGHT, perspective=WHITE):
        self.rank, self.file = pos[0], pos[1]
        self.rankfile = (self.rank, self.file)
        self.size = size
        self.has_piece = has_piece
        self.piece = piece
        self.color = color

        self.default_color = self.color
        self.draw_pos = get_screen_pos((self.file, self.rank))
        self.draw_pos = self.draw_pos if perspective == WHITE else invert_tile(self.draw_pos)
        if self.has_piece:
            self.piece.pos = self.draw_pos
        
        self.border = False
        self.circle = False
        self.active = False
        self.to_be_moved = False
        self.to_be_moved_to = False

    def draw_border(self, screen, pos):
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1], self.size, 3))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1], 3, self.size))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0] + self.size - 3, pos[1], 3, self.size))
        pygame.draw.rect(screen, self.color, pygame.rect.Rect(pos[0], pos[1] + self.size - 3, self.size, 3))
    
    def draw_take(self, screen, pos):
        first = self.size * 1/4
        second = self.size * 3/4

        points = [(pos[0], pos[1]), (pos[0] + first - 1, pos[1]), (pos[0], pos[1] + first - 1)]
        pygame.draw.polygon(screen, Color.DTAKE, points)
        points = [(pos[0] + 2, pos[1] + 2), (pos[0] + first - 1 - 6, pos[1] + 2), (pos[0] + 2, pos[1] + first - 1 - 6)]
        pygame.draw.polygon(screen, Color.LTAKE, points)

        points = [(pos[0] + second - 1, pos[1]), (pos[0] + self.size - 1, pos[1]), (pos[0] + self.size - 1, pos[1] + first - 1)]
        pygame.draw.polygon(screen, Color.DTAKE, points)
        points = [(pos[0] + second - 1 + 6, pos[1] + 2), (pos[0] + self.size - 1 - 2, pos[1] + 2), (pos[0] + self.size - 1 - 2, pos[1] + first - 1 - 6)]
        pygame.draw.polygon(screen, Color.LTAKE, points)

        points = [(pos[0], pos[1] + second - 1), (pos[0], pos[1] + self.size - 1), (pos[0] + first - 1, pos[1] + self.size - 1)]
        pygame.draw.polygon(screen, Color.DTAKE, points)
        points = [(pos[0] + 2, pos[1] + second - 1 + 6), (pos[0] + 2, pos[1] + self.size - 1 - 2), (pos[0] + first - 1 - 6, pos[1] + self.size - 1 - 2)]
        pygame.draw.polygon(screen, Color.LTAKE, points)

        points = [(pos[0] + second - 1, pos[1] + self.size - 1), (pos[0] + self.size - 1, pos[1] + self.size - 1), (pos[0] + self.size - 1, pos[1] + second - 1)]
        pygame.draw.polygon(screen, Color.DTAKE, points)
        points = [(pos[0] + second - 1 + 6, pos[1] + self.size - 1 - 2), (pos[0] + self.size - 1 - 2, pos[1] + self.size - 1 - 2), (pos[0] + self.size - 1 - 2, pos[1] + second - 1 + 6)]
        pygame.draw.polygon(screen, Color.LTAKE, points)

    def draw(self, screen):
        if self.active:
            pygame.draw.rect(screen, Color.YELLOW, pygame.Rect(self.draw_pos[0], self.draw_pos[1], self.size, self.size))
        else:
            pygame.draw.rect(screen, self.default_color, pygame.Rect(self.draw_pos[0], self.draw_pos[1], self.size, self.size))
        if self.circle and not self.has_piece:
            pygame.draw.circle(screen, Color.LTAKE, (self.draw_pos[0] + self.size / 2, self.draw_pos[1] + self.size / 2), self.size / 5)
            pygame.draw.circle(screen, Color.DTAKE, (self.draw_pos[0] + self.size / 2, self.draw_pos[1] + self.size / 2), self.size / 5, 3)
        elif self.circle:
            self.draw_take(screen, self.draw_pos)
        if self.border:
            self.draw_border(screen, self.draw_pos)

        if self.has_piece:
            self.piece.draw(screen)

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