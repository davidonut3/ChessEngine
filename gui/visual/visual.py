from config import *
from visual.tile import Tile
from visual.button import Button
from visual.piece import Piece
from visual.utils import *

class Visual:
    def __init__(self, fen: FenPy, perspective):
        self.fen = fen
        self.perspective = perspective

    def setup(self):
        pygame.init()

        pygame.font.init()
        self.font = pygame.font.SysFont('Consolas', 20)

        self.board_screen = pygame.surface.Surface((BOARD_SIZE, BOARD_SIZE))
        self.screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
        pygame.display.set_caption('Chess')
        pygame.display.set_icon(pygame.image.load(MAIN_LOCATION + '/chess.png'))

        self.board = {(rank, file): Tile((rank, file), TILE_SIZE, False, NO_PIECE, Color.LIGHT if (file % 2 and rank % 2) or (not file % 2 and not rank % 2) else Color.DARK, self.perspective) for file in range(8) for rank in range(8)}
        self.update_board()

        self.save_button = Button((10, 10), 56, 30, self.font, 'Save')
        self.queen_button = Button((70, 10), 22, 30, self.font, 'Q')
        self.rook_button = Button((96, 10), 22, 30, self.font, 'R')
        self.bishop_button = Button((122, 10), 22, 30, self.font, 'B')
        self.knight_button = Button((148, 10), 22, 30, self.font, 'N')

        self.rank_index, self.file_index = load_index_text(self.font)

        self.current_tile = self.board[(0, 0)]
        self.current_tile_active = False
        self.mouse_pos = (0,0)
        self.screen_mouse = (0,0)
        self.board_mouse = (0,0)
        self.in_promotion = False
        self.previous_move = [0, 0]
        self.show_previous_move = False

        pygame.mouse.set_pos(BOARD_SIZE / 2, BOARD_SIZE / 2)
        self.clock = pygame.time.Clock()
        self.mouse_pos = pygame.mouse.get_pos()
        self.mouse_down = False

    def update_board(self):
        board = self.fen.to_visual()
        for rank in range(8):
            for file in range(8):
                piece = board[rank][file]
                tile = self.board[(rank, file)]
                if piece == '-':
                    tile.has_piece = False
                    tile.piece = NO_PIECE
                else:
                    piece_name = letter_to_name[piece]
                    piece_type = letter_to_type[piece]
                    piece_bounds = letter_to_bounds[piece]
                    tile.has_piece = True
                    tile.set_piece(Piece(piece_name, piece_type, self.perspective, piece_bounds))

    def draw_board(self, screen):
        if self.show_previous_move:
            start: Tile = self.previous_move[0]
            end: Tile = self.previous_move[1]
            start.set_display(Color.LGREEN, True, start.circle)
            end.set_display(Color.DGREEN, True, end.circle)
        for rank in range(8):
            for file in range(8):
                self.board[(rank, file)].draw(screen)
        if self.current_tile_active:
            self.current_tile.piece.draw(screen)

    def run_static(self, match):
        self.update_board()

        while True:
            self.set_mouse()
            mouse_file, mouse_rank = set_in_bounds(self.board_mouse)
            pygame.display.set_caption(file_to_string[mouse_file] + rank_to_string[mouse_rank])

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return False
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.mouse_down = True
                if event.type == pygame.MOUSEBUTTONUP:
                    self.mouse_down = False

            self.update_save()

            self.screen.fill(Color.BG)
            self.save_button.draw(self.screen)
            self.queen_button.draw(self.screen)
            self.rook_button.draw(self.screen)
            self.bishop_button.draw(self.screen)
            self.knight_button.draw(self.screen)
            self.board_screen.fill(Color.WHITE)

            self.draw_board(self.board_screen)
            self.draw_file_rank(self.screen)

            self.screen.blit(self.board_screen, (BOARD_X, BOARD_Y))

            pygame.display.update()
            self.clock.tick(60)

            if match.get_move():
                return True

    def get_move(self, turn):
        self.update_board()

        while True:
            self.set_mouse()
            mouse_file, mouse_rank = set_in_bounds(self.board_mouse)
            pygame.display.set_caption(file_to_string[mouse_file] + rank_to_string[mouse_rank])

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return False
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.mouse_down = True
                    if not self.current_tile_active:
                        self.select_piece(turn)
                if event.type == pygame.MOUSEBUTTONUP:
                    self.mouse_down = False
                    self.mouse_down = False
                    if self.current_tile_active:
                        self.board_mouse = set_in_bounds(self.board_mouse)
                        start = self.current_tile
                        end = self.board[(self.board_mouse[1], self.board_mouse[0])]
                        move = move_to_lan(start.rankfile, end.rankfile)
                        legal = self.fen.is_legal_move_lan(move)
                        self.place_piece(start.rankfile, end.rankfile, legal)
                        if legal:
                            if start.piece.name == 'PAWN':
                                if (turn == WHITE and end.rankfile[0] == 0) or (turn == BLACK and end.rankfile[0] == 7):
                                    promotion = self.get_promotion()
                                    if not promotion:
                                        return False
                                    else:
                                        move += promotion
                            return move

            if self.current_tile_active:
                self.current_tile.piece.center_image_at(self.screen_mouse)

            self.update_save()

            self.screen.fill(Color.BG)
            self.save_button.draw(self.screen)
            self.queen_button.draw(self.screen)
            self.rook_button.draw(self.screen)
            self.bishop_button.draw(self.screen)
            self.knight_button.draw(self.screen)
            self.board_screen.fill(Color.WHITE)

            self.draw_board(self.board_screen)
            self.draw_file_rank(self.screen)

            self.screen.blit(self.board_screen, (BOARD_X, BOARD_Y))

            pygame.display.update()
            self.clock.tick(60)

    def show_board(self):
        self.update_board()

        while True:
            self.set_mouse()
            mouse_file, mouse_rank = set_in_bounds(self.board_mouse)
            pygame.display.set_caption(file_to_string[mouse_file] + rank_to_string[mouse_rank])

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return False
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.mouse_down = True
                if event.type == pygame.MOUSEBUTTONUP:
                    self.mouse_down = False

            self.update_save()

            self.screen.fill(Color.BG)
            self.save_button.draw(self.screen)
            self.queen_button.draw(self.screen)
            self.rook_button.draw(self.screen)
            self.bishop_button.draw(self.screen)
            self.knight_button.draw(self.screen)
            self.board_screen.fill(Color.WHITE)

            self.draw_board(self.board_screen)
            self.draw_file_rank(self.screen)

            self.screen.blit(self.board_screen, (BOARD_X, BOARD_Y))

            pygame.display.update()
            self.clock.tick(60)

    def get_promotion(self):
        while True:
            self.set_mouse()
            mouse_file, mouse_rank = set_in_bounds(self.board_mouse)
            pygame.display.set_caption(file_to_string[mouse_file] + rank_to_string[mouse_rank])

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    return False
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.mouse_down = True
                if event.type == pygame.MOUSEBUTTONUP:
                    self.mouse_down = False

            self.update_save()
            queen_button = self.queen_button.update(self.mouse_pos, self.mouse_down)
            if queen_button:
                return 'q'
            rook_button = self.rook_button.update(self.mouse_pos, self.mouse_down)
            if rook_button:
                return 'r'
            bishop_button = self.bishop_button.update(self.mouse_pos, self.mouse_down)
            if bishop_button:
                return 'b'
            knight_button = self.knight_button.update(self.mouse_pos, self.mouse_down)
            if knight_button:
                return 'n'

            self.screen.fill(Color.BG)
            self.save_button.draw(self.screen)
            self.queen_button.draw(self.screen)
            self.rook_button.draw(self.screen)
            self.bishop_button.draw(self.screen)
            self.knight_button.draw(self.screen)
            self.board_screen.fill(Color.WHITE)

            self.draw_board(self.board_screen)
            self.draw_file_rank(self.screen)

            self.screen.blit(self.board_screen, (BOARD_X, BOARD_Y))

            pygame.display.update()
            self.clock.tick(60)

    def select_piece(self, turn):
        if in_bounds(self.board_mouse):
            selected_tile: Tile = self.board[(self.board_mouse[1], self.board_mouse[0])]
            if selected_tile.has_piece and selected_tile.piece.type == turn:
                self.current_tile = selected_tile
                self.current_tile_active = True
                self.show_possible_moves()
                self.current_tile.set_active()

    def reset_tiles(self, full=False):
        for rank in range(8):
            for file in range(8):
                tile = self.board[(rank, file)]
                if tile.has_piece:
                    tile.piece.pos = tile.draw_pos
                if not tile in self.previous_move or full:
                    tile.reset()
                elif tile in self.previous_move:
                    tile.set_display(tile.default_color, tile.border, False)
    
    def place_piece(self, start, end, legal):
        self.reset_tiles()
        if legal:
            if self.show_previous_move:
                self.previous_move[0].reset()
                self.previous_move[1].reset()
            else:
                self.show_previous_move = True
            self.previous_move[0] = self.board[start]
            self.previous_move[1] = self.board[end]
        self.current_tile_active = False

    def show_possible_moves(self):
        possible_moves = self.fen.get_possible_moves_tile(tile_to_string(self.current_tile.rankfile))
        for lan in possible_moves:
            tile: Tile = self.board[lan_to_move(lan)[1]]
            tile.set_display(tile.color, tile.border, True)

    def draw_file_rank(self, surface):
        for i, text in enumerate(self.rank_index):
            surface.blit(text, (BOARD_X - 15, BOARD_Y + 5 + i * TILE_SIZE))
        for i, text in enumerate(self.file_index):
            surface.blit(text, (2 * BOARD_X - 15 + i * TILE_SIZE, BOARD_Y + BOARD_SIZE + 5))

    def set_mouse(self):
        self.mouse_pos = pygame.mouse.get_pos()
        self.screen_mouse = convert_mouse(self.mouse_pos, self.perspective)
        self.board_mouse = get_board_pos(self.screen_mouse)

    def update_save(self):
        button_clicked = self.save_button.update(self.mouse_pos, self.mouse_down)
        if button_clicked:
            print(f'The current fen is: {self.fen.to_string()}')