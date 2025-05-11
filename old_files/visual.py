from config import *
from visual.tile import Tile
from visual.button import Button
from visual.utils import *
from debugger import Debugger
from storage.fen import Fen

class Visual:
    def __init__(self, fen: Fen, perspective):
        self.fen = fen
        self.perspective = perspective

    def setup(self):
        pygame.init()

        pygame.font.init()
        self.font = pygame.font.SysFont('Consolas', 20)

        self.board_screen = pygame.surface.Surface((BOARD_SIZE, BOARD_SIZE))
        self.screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
        pygame.display.set_caption('Chess')
        pygame.display.set_icon(pygame.image.load(getcwd() + '/chess.png'))
        
        empty_board = [[Tile((i,j), TILE_SIZE, False, NO_PIECE, Color.LIGHT if (i % 2 and j % 2) or (not i % 2 and not j % 2) else Color.DARK) for i in range(8)] for j in range(8)]
        self.board = self.fen.fen_to_visual(empty_board, self.perspective)
        self.save_button = Button((10, 10), 56, 30, self.font, 'Save')

        self.current_tile = self.board[0][0]
        self.current_tile_active = False
        self.screen_mouse = (0,0)
        self.board_mouse = (0,0)
        self.in_promotion = False
        self.previous_move = [0, 0]
        self.show_previous_move = False

        pygame.mouse.set_pos(BOARD_SIZE / 2, BOARD_SIZE / 2)
        self.clock = pygame.time.Clock()
        self.mouse_pos = pygame.mouse.get_pos()
        self.mouse_down = False
        self.running = True
    
    def user_input(self, turn):
        mouse_pos = pygame.mouse.get_pos()
        self.screen_mouse = convert_mouse(mouse_pos, self.perspective)
        self.board_mouse = get_board_pos(self.screen_mouse)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                self.running = False
            if event.type == pygame.MOUSEBUTTONDOWN:
                self.mouse_down = True
                if not self.current_tile_active:
                    self.select_piece(turn)
            if event.type == pygame.MOUSEBUTTONUP:
                self.mouse_down = False
                if self.current_tile_active:
                    start = self.current_tile.pos
                    self.board_mouse = set_in_bounds(self.board_mouse)
                    end = self.board[self.board_mouse[1]][self.board_mouse[0]].pos
                    self.place_piece(self.current_tile.pos, False)
                    return [start[1], start[0]], [end[1], end[0]]
        
        return False, False
    
    def update_fen(self, pos):
        print(self.fen)
        self.board = self.fen.fen_to_visual(self.board, self.perspective)
        self.place_piece([pos[1], pos[0]], True)

    def draw(self):
        self.screen.fill(Color.BG)
        self.save_button.draw(self.screen)
        self.board_screen.fill(Color.WHITE)

        self.draw_board(self.board_screen)

        self.screen.blit(self.board_screen, (BOARD_X, BOARD_Y))

        pygame.display.update()
        self.clock.tick(60)

    def update(self, debugger: Debugger):
        if self.current_tile_active:
            self.current_tile.piece.center_image_at(self.screen_mouse)

        button_clicked = self.save_button.update(pygame.mouse.get_pos(), self.mouse_down)
        if button_clicked:
            debugger.log(f'Current board is: {self.fen}')

        self.board = self.fen.fen_to_visual(self.board, self.perspective)

    def end(self):
        pygame.quit()

    def select_piece(self, turn):
        if in_bounds(self.board_mouse):
            selected_tile: Tile = self.board[self.board_mouse[1]][self.board_mouse[0]]
            if selected_tile.has_piece and selected_tile.piece.type == turn:
                self.current_tile = selected_tile
                self.current_tile_active = True
                self.show_possible_moves()
                self.current_tile.set_active()

    def place_piece(self, pos, legal):
        self.reset_colors()
        if legal:
            if self.show_previous_move:
                self.previous_move[0].reset()
                self.previous_move[1].reset()
            else:
                self.show_previous_move = True
            self.previous_move[0] = self.current_tile
            self.previous_move[1] = self.board[pos[1]][pos[0]]
        self.current_tile_active = False

    def draw_board(self, screen):
        if self.show_previous_move:
            start = self.previous_move[0]
            end = self.previous_move[1]
            start.set_display(Color.LGREEN, True, start.circle)
            end.set_display(Color.DGREEN, True, end.circle)
        for row in self.board:
            for tile in row:
                tile.draw(screen)
        for row in self.board:
            for tile in row:
                if tile.has_piece:
                    tile.piece.draw(screen)
        if self.current_tile_active:
            self.current_tile.piece.draw(screen)

    def show_possible_moves(self):
        possible_moves = self.fen.get_possible_moves(self.current_tile)
        for pos in possible_moves:
            tile = self.board[pos[0]][pos[1]]
            tile.set_display(tile.color, tile.border, True)
    
    def reset_colors(self, full=False):
        for row in self.board:
            for tile in row:
                if not tile in self.previous_move or full:
                    tile.reset()
                elif tile in self.previous_move:
                    tile.set_display(tile.default_color, tile.border, False)